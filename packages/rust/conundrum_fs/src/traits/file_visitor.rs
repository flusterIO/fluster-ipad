use conundrum::lang::runtime::run_conundrum::ParseConundrumOptions;
use ignore::{DirEntry, Error, WalkState};
use std::path::PathBuf;

use crate::traits::parsable_file::ParsableFile;

pub trait FileVisitor: ParsableFile + Sized {
    /// Takes the **absolute** path of where the parsed output should be
    /// written.
    fn parse_to_path<'a>(output_path: PathBuf,
                         opts: &'a ParseConundrumOptions)
                         -> Box<dyn FnMut(Result<DirEntry, Error>) -> WalkState + Send + 'a> {
        Box::new(move |entry| {
            if let Ok(res) = entry {
                if let Ok(file_res) = Self::from_path(res.path().to_path_buf(), opts) {
                    if let Ok(file_content) = file_res.as_file_content() {
                        if let Err(err) = std::fs::write(output_path.clone(), file_content) {
                            eprintln!("Error: {}", err);
                        }
                    }
                }
            }
            WalkState::Continue
        })
    }
}

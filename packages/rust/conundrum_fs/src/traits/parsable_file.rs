use std::path::PathBuf;

use conundrum::lang::runtime::{
    run_conundrum::ParseConundrumOptions,
    state::{
        conundrum_error::ConundrumError,
        conundrum_error_variant::{ConundrumErrorVariant, ConundrumResult},
    },
};

pub trait ParsableFile {
    fn as_file_content(&self) -> ConundrumResult<String>;
    fn parse_string(content: String, absolute_path: &PathBuf, opts: &ParseConundrumOptions) -> ConundrumResult<Self>
        where Self: Sized;
    fn from_path(path: PathBuf, opts: &ParseConundrumOptions) -> ConundrumResult<Self>
        where Self: Sized {
        match std::fs::read_to_string(&path) {
            Ok(o) => Self::parse_string(o, &path, opts),
            Err(err) => {
                Err(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("File system error",
                                                                                                    format!("Conundrum encoutered an error while attempting to parse the following path: \n{}\nError: {}", path.to_str().unwrap_or("unknown"),
                                                                                                        err).as_str(),
                                                                                                    )
                        ))
            }
        }
    }
}

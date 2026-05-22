use std::path::PathBuf;

use conundrum::lang::{
    constants::{file_names::CDRM_IGNORE_FILE_NAME, file_types::ParsableFileType},
    runtime::state::conundrum_error_variant::ConundrumResult,
};
use ignore::{DirEntry, Error, WalkBuilder, WalkState};

use crate::traits::config_file::ConfigFile;

pub struct FileCollectionRequest<'a, T, J>
    where T: FnMut() -> Box<dyn FnMut(Result<DirEntry, Error>) -> WalkState + Send + 'a>,
          J: Fn(DirEntry) -> bool {
    /// Return true if the file should be parsed based on the file's filepath.
    pub should_parse: J,
    /// The `root` of the query to use for recursive searching.
    pub root: PathBuf,
    pub respect_gitignore: bool,
    pub callback: T,
}

pub trait FileCollectionVisitor<'a, T, J>: ConfigFile
    where T: FnMut() -> Box<dyn FnMut(Result<DirEntry, Error>) -> WalkState + Send + 'a>,
          J: Fn(DirEntry) -> bool {
    fn visit_files(&self, req: FileCollectionRequest<'a, T, J>) -> ConundrumResult<()> {
        let w = WalkBuilder::new(req.root).add_custom_ignore_filename(CDRM_IGNORE_FILE_NAME)
                                          .git_ignore(req.respect_gitignore)
                                          .build_parallel();
        w.run(req.callback);
        Ok(())
    }
}

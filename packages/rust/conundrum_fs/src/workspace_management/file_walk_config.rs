use std::path::Path;

use conundrum::lang::constants::file_types::ParsableFileType;

pub struct FileWalkConfig {
    pub root: String,
    pub respect_git_ignore: bool,
    pub ignore_hidden: bool,
    pub file_type: ParsableFileType,
}

use std::path::Path;

use conundrum::lang::constants::file_types::ParsableFileType;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct FileWalkConfig {
    pub root: String,
    pub respect_git_ignore: bool,
    pub ignore_hidden: bool,
    pub file_type: ParsableFileType,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct FileCountConfig {
    pub root: String,
    pub respect_gitignore: bool,
    pub ignore_hidden: bool,
}

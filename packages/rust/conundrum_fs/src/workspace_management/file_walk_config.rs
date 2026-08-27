#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct FileWalkConfig {
    pub root: String,
    pub respect_git_ignore: bool,
    pub ignore_hidden: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct FileCountConfig {
    pub root: String,
    pub respect_gitignore: bool,
    pub ignore_hidden: bool,
}

use conundrum::lang::constants::file_types::ParsableFileType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
pub enum PathVariant {
    File,
    Dir,
}

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type)]
pub struct FSPathSimpleResult {
    /// The _relative_ path
    pub path: String,
    pub variant: PathVariant,
    pub parsable: Option<ParsableFileType>,
}

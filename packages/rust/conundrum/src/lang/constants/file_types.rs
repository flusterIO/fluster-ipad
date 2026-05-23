use serde::{Deserialize, Serialize};

/// All string representations must match the file type, without the leading
/// period.
#[derive(Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString, Clone, PartialEq, Eq)]
pub enum ParsableFileType {
    #[serde(rename = "cdrm")]
    #[strum(to_string = "cdrm")]
    Cdrm,
}

impl ParsableFileType {
    pub fn extension_is_conundrum_file(ext: &str) -> bool {
        [Self::Cdrm.to_string().as_str(), "mdx", "md"].contains(&ext)
    }
}

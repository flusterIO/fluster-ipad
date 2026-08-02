use serde::{Deserialize, Serialize};
use specta::Type;
use strum::EnumIter;

use crate::ecosystem::error_handling::conundrum_fs_error::ConundrumFSError;

/// All string representations must match the file type, without the leading
/// period.
#[derive(Serialize,
           Deserialize,
           strum_macros::Display,
           strum_macros::EnumString,
           Clone,
           PartialEq,
           Eq,
           EnumIter,
           Hash,
           Debug,
           Type)]
pub enum ParsableFileType {
    #[serde(rename = "cdrm")]
    #[strum(to_string = "cdrm")]
    Cdrm,
    #[serde(rename = "md")]
    #[strum(to_string = "md")]
    Markdown,
    #[serde(rename = "mdx")]
    #[strum(to_string = "mdx")]
    Mdx,
    #[serde(rename = "typst")]
    #[strum(to_string = "typst")]
    Typst,
}

impl TryFrom<String> for ParsableFileType {
    type Error = ConundrumFSError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "cdrm" => Ok(Self::Cdrm),
            "md" => Ok(Self::Markdown),
            "mdx" => Ok(Self::Mdx),
            "typst" => Ok(Self::Typst),
            _ => Err(ConundrumFSError::UnsupportedFileExtension(value.clone())),
        }
    }
}

impl ParsableFileType {
    pub fn extension_is_conundrum_file(ext: &str) -> bool {
        [Self::Cdrm.to_string().as_str(), "mdx", "md"].contains(&ext)
    }

    /// Returns the file type name and glob for the ignore crate.
    pub fn to_ignore_types(&self) -> (&'static str, &'static str) {
        match self {
            Self::Cdrm => ("conundrum", "*.cdrm"),
            Self::Mdx => ("mdx", "*.mdx"),
            Self::Markdown => ("markdown", "*.md"),
            Self::Typst => ("typst", "*.typst"),
        }
    }
}

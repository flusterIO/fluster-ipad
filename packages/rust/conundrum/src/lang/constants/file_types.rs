use serde::{Deserialize, Serialize};

/// All string representations must match the file type, without the leading
/// period.
#[derive(Serialize, Deserialize, strum_macros::Display, strum_macros::EnumString, Clone, PartialEq, Eq)]
pub enum ParsableFileType {
    #[serde(rename = "cdrm")]
    #[strum(to_string = "cdrm")]
    Cdrm,
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// ## Deprecated
/// Moved to the config package.
#[derive(Serialize, Default, Deserialize, strum_macros::Display, Clone, Eq, PartialEq, JsonSchema)]
pub enum ConundrumWebProjectBuilder {
    /// When using the **Next.js** provider, your output should point to a file.
    /// If it points to a directory, a single `cdrm_output.json` file will
    /// be generated in that directory.
    ///
    /// Unfortunately, there currently isn't support for multiple files. I plan
    /// to get around to it, but my to-do list is _long_.
    #[serde(rename = "next")]
    #[default]
    Next,
}

use serde::{Deserialize, Serialize};

use crate::lang::runtime::state::parse_state::ConundrumCompileTarget;

#[derive(Serialize, Deserialize, Clone)]
pub struct ConundrumDocsNextProviderEntry {
    pub relative_path: String,
    pub format: ConundrumCompileTarget,
}

pub fn default_cdrm_path() -> Option<String> {
    Some(String::from("conundrum"))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConundrumDocsNextProviderConfig {
    /// The path, relative to the project root of the output directory. This is
    /// the same path as what is passed in to the cli during generation, if
    /// you are using a non-default path.
    #[serde(default = "default_cdrm_path")]
    pub source_path: Option<String>,
    pub docs: Vec<ConundrumDocsNextProviderEntry>,
}

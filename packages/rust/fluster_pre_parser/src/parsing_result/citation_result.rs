use serde::{Deserialize, Serialize};

#[derive(uniffi::Record, Serialize, Deserialize, Clone)]
pub struct CitationResult {
    /// The parsed citation key.
    pub citation_key: String,
    /// The complete bibtex entry
    pub index: u8,
}

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, uniffi::Record, Clone)]
pub struct CitationSummaryData {
    citation_key: String,
    body: String,
}

#[derive(uniffi::Record, Debug, Serialize, Deserialize, Clone)]
pub struct CitationResult {
    /// The parsed citation key.
    pub citation_key: String,
    /// The complete bibtex entry
    pub idx: u8,
}

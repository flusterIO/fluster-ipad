use serde::{Deserialize, Serialize};

use crate::parse::by_regex::parse_mdx_by_regex::SwiftDataCitationSummary;

#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct CitationResult {
    /// The parsed citation key.
    pub citation_key: String,
    /// The complete bibtex entry
    pub entry: String,
}

impl CitationResult {
    pub fn new(
        citation_key: &str,
        entries: &Vec<SwiftDataCitationSummary>,
    ) -> Option<CitationResult> {
        if let Some(entry) = entries.iter().find(|x| x.citation_key == citation_key) {
            Some(CitationResult {
                citation_key: citation_key.to_string(),
                entry: entry.body.clone(),
            })
        } else {
            None
        }
    }
}

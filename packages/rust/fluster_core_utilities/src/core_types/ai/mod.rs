use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct CodeBlockParsingResult {
    pub full_match: String,
    pub language_tag: String,
    pub block_content: String,
}

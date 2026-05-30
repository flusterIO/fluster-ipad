use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Both the label and body fields are ***un-compiled*** Conundrum content.
#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug, uniffi::Record)]
pub struct DictionaryEntryResult {
    pub label: String,
    pub body: String,
}

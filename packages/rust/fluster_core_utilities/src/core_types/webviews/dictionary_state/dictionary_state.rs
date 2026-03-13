use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct WebviewDictionaryEntry {
    pub label: String,
    pub body: String,
    pub origin_note_id: Option<String>,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct DictionaryState {
    pub entries: Vec<WebviewDictionaryEntry>,
}

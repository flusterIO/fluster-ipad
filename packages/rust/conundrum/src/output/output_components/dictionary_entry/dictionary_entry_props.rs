use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize)]
pub struct DictionaryEntryResultData {
    pub label: String,
    pub note_id: Option<String>,
}

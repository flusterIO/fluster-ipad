use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, Serialize, Deserialize, strum_macros::Display)]
pub enum DictionaryStateActions {
    #[serde(rename = "set-note-details")]
    #[strum(to_string = "set-note-details")]
    SetDictionaryEntries,
}

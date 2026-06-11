use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, Serialize, Deserialize, strum_macros::Display)]
pub enum DictionaryStateActions {
    #[serde(rename = "set-dict-entriea")]
    #[strum(to_string = "set-dict-entriea")]
    SetDictionaryEntries,
}

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug, uniffi::Record)]
pub struct DictionaryEntryResult {
    pub label: String,
    pub body: String,
}

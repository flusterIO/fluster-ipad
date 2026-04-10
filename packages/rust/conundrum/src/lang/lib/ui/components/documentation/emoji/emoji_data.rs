use serde::Serialize;
use tabled::Tabled;

#[typeshare::typeshare]
#[derive(Debug, Tabled, Serialize, uniffi::Record, Clone)]
pub struct EmojiData {
    pub name: String,
    pub svg: String,
}

impl EmojiData {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| String::from("{}"))
    }
}

use serde::Serialize;
use tabled::Tabled;

#[typeshare::typeshare]
#[derive(Debug, Tabled, Serialize, uniffi::Record)]
pub struct EmojiData {
    pub name: String,
    pub svg: String,
}

use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uniffi::Record;

#[typeshare]
#[derive(uniffi::Enum, strum_macros::Display, Serialize, Deserialize, Clone)]
pub enum SnippetCategoryId {
    #[serde(rename = "icons")]
    #[strum(to_string = "icons")]
    Icons,
    #[serde(rename = "math")]
    #[strum(to_string = "math")]
    Math,
    #[serde(rename = "components")]
    #[strum(to_string = "components")]
    Components,
    #[serde(rename = "componentVariants")]
    #[strum(to_string = "componentVariants")]
    ComponentVariants,
    #[serde(rename = "emojis")]
    #[strum(to_string = "emojis")]
    Emojis,
}

#[typeshare]
#[derive(Record, Serialize, Deserialize, Clone)]
pub struct SnippetState {
    #[serde(rename = "includeEmojiSnippets")]
    pub include_emoji_snippets: bool,
}

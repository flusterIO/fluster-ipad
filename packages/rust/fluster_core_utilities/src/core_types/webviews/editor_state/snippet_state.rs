use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uniffi::Record;

#[typeshare]
#[derive(Record, Serialize, Deserialize)]
pub struct SnippetState {
    #[serde(rename = "includeEmojiSnippets")]
    pub include_emoji_snippets: bool,
}

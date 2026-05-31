
#[typeshare::typeshare]
#[derive(strum_macros::Display, serde::Serialize, serde::Deserialize)]
pub enum CopyToClipboardSource {
    #[serde(rename = "emoji-name")]
    #[strum(to_string = "emoji-name")]
    EmojiName,
}

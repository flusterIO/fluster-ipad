#[derive(strum_macros::Display, uniffi::Enum, Serialize, Deserialize, Clone)]
pub enum ConundrumBrowserEvents {
    #[serde(rename = "cdrm-codeblock-copy")]
    #[strum(to_string = "cdrm-codeblock-copy")]
    CodeblockCopied,
    #[serde(rename = "cdrm-note-id-link-click")]
    #[strum(to_string = "cdrm-note-id-link-click")]
    NoteLinkClick,
}

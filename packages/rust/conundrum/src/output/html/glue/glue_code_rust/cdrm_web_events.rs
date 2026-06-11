use serde::{Deserialize, Serialize};

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, uniffi::Enum, strum_macros::Display)]
pub enum ConundrumWebEvents {
    /// Useful for things like resizable pains, to more easily emit your own
    /// resize event that Conundrum will handle to update the components
    /// accordingly.
    #[serde(rename = "cdrm-manual-resize")]
    #[strum(to_string = "cdrm-manual-resize")]
    ManualResize,
    #[serde(rename = "cdrm-content-loaded")]
    #[strum(to_string = "cdrm-content-loaded")]
    CdrmContentLoaded,
    #[serde(rename = "cdrm-codeblock-copy")]
    #[strum(to_string = "cdrm-codeblock-copy")]
    CodeblockCopied,
    #[serde(rename = "cdrm-note-id-link-click")]
    #[strum(to_string = "cdrm-note-id-link-click")]
    NoteLinkClick,
    #[serde(rename = "cdrm-content-copied")]
    #[strum(to_string = "cdrm-content-copied")]
    CopyToClipboard,
    #[serde(rename = "dictionary-entry-click")]
    #[strum(to_string = "dictionary-entry-click")]
    DictionaryEntryLabelClick,
    #[serde(rename = "toc-item-click")]
    #[strum(to_string = "toc-item-click")]
    TocItemClick,
    #[serde(rename = "tag-click")]
    #[strum(to_string = "tag-click")]
    TagClick,
}

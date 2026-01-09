use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// From typescript to swift.
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum MdxPreviewWebviewActions {
    #[serde(rename = "request-note-data")]
    #[strum(to_string = "request-note-data")]
    RequestNoteData,
    #[serde(rename = "set-preview-webview-loaded")]
    #[strum(to_string = "set-preview-webview-loaded")]
    SetWebviewLoaded,
    #[serde(rename = "view-note-by-user-def-id")]
    #[strum(to_string = "view-note-by-user-def-id")]
    ViewNoteByUserDefinedId,
}

/// From swift to typescript
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum MdxPreviewWebviewEvents {
    #[serde(rename = "set-initial-color-scheme")]
    #[strum(to_string = "set-initial-color-scheme")]
    SetInitialColorScheme,
    #[serde(rename = "set-mdx-preview-content")]
    #[strum(to_string = "set-mdx-preview-content")]
    SetPreviewContent,
}

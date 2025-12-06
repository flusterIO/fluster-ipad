use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// From typescript to swift.
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum NoteDetailWebviewActions {
    #[serde(rename = "request-note-detail-data")]
    #[strum(to_string = "request-note-detail-data")]
    RequestNoteDetailData,
    #[serde(rename = "set-note-detail-webview-loaded")]
    #[strum(to_string = "set-note-detail-webview-loaded")]
    SetWebviewLoaded,
}

/// From swift to typescript
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum NoteDetailWebviewEvents {
    #[serde(rename = "set-initial-color-scheme")]
    #[strum(to_string = "set-initial-color-scheme")]
    SetInitialColorScheme,
    #[serde(rename = "set-note-details")]
    #[strum(to_string = "set-note-details")]
    SetNoteDetails,
}

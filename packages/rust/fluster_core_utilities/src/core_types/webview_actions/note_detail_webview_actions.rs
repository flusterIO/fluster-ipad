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
    #[serde(rename = "handle-tag-click")]
    #[strum(to_string = "handle-tag-click")]
    HandleTagClick,
    #[serde(rename = "handle-topic-click")]
    #[strum(to_string = "handle-topic-click")]
    HandleTopicClick,
    #[serde(rename = "handle-subject-click")]
    #[strum(to_string = "handle-subject-click")]
    HandleSubjectClick,
    #[serde(rename = "handle-citation-click")]
    #[strum(to_string = "handle-citation-click")]
    HandleCitationClick,
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

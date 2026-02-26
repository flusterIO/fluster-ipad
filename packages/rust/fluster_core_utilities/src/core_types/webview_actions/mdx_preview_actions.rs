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
    #[serde(rename = "view-note-by--id")]
    #[strum(to_string = "view-note-by--id")]
    ViewNoteById,
    #[serde(rename = "tag-click-event")]
    #[strum(to_string = "tag-click-event")]
    OnTagClick,
    #[serde(rename = "handle-topic-click")]
    #[strum(to_string = "handle-topic-click")]
    OnTopicClick,
    #[serde(rename = "handle-subject-click")]
    #[strum(to_string = "handle-subject-click")]
    OnSubjectClick,
    #[serde(rename = "handle-citation-click")]
    #[strum(to_string = "handle-citation-click")]
    OnCitationClick,
    #[serde(rename = "show-user-facing-notification")]
    #[strum(to_string = "show-user-facing-notification")]
    ShowUserFacingNotification,
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

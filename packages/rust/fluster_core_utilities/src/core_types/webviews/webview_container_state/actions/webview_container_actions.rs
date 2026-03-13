use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, strum_macros::Display, Serialize, Deserialize)]
pub enum WebviewContainerActions {
    #[serde(rename = "set-fluster-theme")]
    #[strum(to_string = "set-fluster-theme")]
    SetFlusterTheme,
    #[serde(rename = "set-dark-mode")]
    #[strum(to_string = "set-dark-mode")]
    SetDarkMode,
    #[serde(rename = "handle-note-deleted")]
    #[strum(to_string = "handle-note-deleted")]
    HandleNoteDeleted,
    #[serde(rename = "set-webview-fontsize")]
    #[strum(to_string = "set-webview-fontsize")]
    SetWebviewFontSize,
}

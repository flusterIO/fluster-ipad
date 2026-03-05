use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, strum_macros::Display, Serialize, Deserialize)]
pub enum EditorStateActions {
    #[serde(rename = "set-editor-save-method")]
    #[strum(to_string = "set-editor-save-method")]
    SetEditorSaveMethod,
    #[serde(rename = "set-initial-editor-state")]
    #[strum(to_string = "set-initial-editor-state")]
    SetInitialEditorState,
}

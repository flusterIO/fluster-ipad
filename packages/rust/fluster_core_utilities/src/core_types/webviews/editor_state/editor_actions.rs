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
    #[serde(rename = "set-parsed-editor-content")]
    #[strum(to_string = "set-parsed-editor-content")]
    /// This is the 'onChange' method that's executed _after_ the content is parsed.
    SetParsedEditorContent,
    #[serde(rename = "set-initial-editor-keymap")]
    #[strum(to_string = "set-initial-editor-keymap")]
    SetEditorKeymap,
}

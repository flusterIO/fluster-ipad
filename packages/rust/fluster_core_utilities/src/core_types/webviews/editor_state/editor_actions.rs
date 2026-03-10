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
    SetParsedEditorContent,
    #[serde(rename = "set-initial-editor-keymap")]
    #[strum(to_string = "set-initial-editor-keymap")]
    SetEditorKeymap,
    #[serde(rename = "set-editor-theme")]
    #[strum(to_string = "set-editor-theme")]
    SetEditorTheme,
    #[serde(rename = "set-autosave-timeout")]
    #[strum(to_string = "set-autosave-timeout")]
    SetAutoSaveTimeout,
    #[serde(rename = "set-base-keymap")]
    #[strum(to_string = "set-base-keymap")]
    SetBaseKeymap,
    #[serde(rename = "set-editor-tags")]
    #[strum(to_string = "set-editor-tags")]
    SetEditorTags,
    #[serde(rename = "set-all-citation-ids")]
    #[strum(to_string = "set-all-citation-ids")]
    SetAllCitationIds,
    #[serde(rename = "set-lock-editor-scroll-to-prev")]
    #[strum(to_string = "set-lock-editor-scroll-to-prev")]
    SetLockEditorScrollToPreview,
    #[serde(rename = "set-snippet-props")]
    #[strum(to_string = "set-snippet-props")]
    SetSnippetProps,
}

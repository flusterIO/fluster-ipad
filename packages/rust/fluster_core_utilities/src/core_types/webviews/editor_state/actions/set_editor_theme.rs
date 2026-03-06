use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::{
    editor_actions::EditorStateActions, editor_theme::CodeEditorTheme,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetEditorThemePayload {
    theme: CodeEditorTheme,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetEditorThemeAction {
    pub r#type: EditorStateActions,
    pub payload: SetEditorThemePayload,
}

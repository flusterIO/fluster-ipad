use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::{
    editor_actions::EditorStateActions, editor_theme::CodeEditorTheme,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetEditorThemeLightPayload {
    pub theme_light: CodeEditorTheme,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetEditorThemeLightAction {
    pub r#type: EditorStateActions,
    pub payload: SetEditorThemeLightPayload,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetEditorThemeDarkPayload {
    pub theme_dark: CodeEditorTheme,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetEditorThemeDarkAction {
    pub r#type: EditorStateActions,
    pub payload: SetEditorThemeDarkPayload,
}

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::{
    editor_save_method::EditorSaveMethod, editor_state::editor_actions::EditorStateActions,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetEditorSaveMethodEditorAction {
    pub r#type: EditorStateActions,
    pub payload: EditorSaveMethod,
}

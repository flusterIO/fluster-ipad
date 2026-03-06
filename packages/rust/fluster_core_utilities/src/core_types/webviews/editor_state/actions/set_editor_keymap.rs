use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::{
    editor_actions::EditorStateActions, editor_keymap::CodeEditorKeymap,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetEditorKeymapPayload {
    keymap: CodeEditorKeymap,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetEditorKeymapAction {
    pub r#type: EditorStateActions,
    pub payload: SetEditorKeymapPayload,
}

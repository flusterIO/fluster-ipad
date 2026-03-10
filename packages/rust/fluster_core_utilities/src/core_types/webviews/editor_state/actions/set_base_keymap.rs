use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::{
    editor_actions::EditorStateActions, editor_state::CodeEditorBaseKeymap,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetBaseKeymapPayload {
    base_keymap: CodeEditorBaseKeymap,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetBaseKeymapAction {
    pub r#type: EditorStateActions,
    pub payload: SetBaseKeymapPayload,
}

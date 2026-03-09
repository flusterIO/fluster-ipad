use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::{
    editor_actions::EditorStateActions, editor_initial_state::EditorBasedWebviewInitialState,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetEditorInitialStateEditorAction {
    pub r#type: EditorStateActions,
    pub payload: EditorBasedWebviewInitialState,
}

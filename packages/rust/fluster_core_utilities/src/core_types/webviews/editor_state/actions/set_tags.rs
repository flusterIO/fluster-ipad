use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::{
    editor_actions::EditorStateActions, editor_state::EditorTag,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetEditorTagsPayload {
    tags: Vec<EditorTag>,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetEditorTagsAction {
    pub r#type: EditorStateActions,
    pub payload: SetEditorTagsPayload,
}

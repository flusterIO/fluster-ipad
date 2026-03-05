use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uniffi::Record;

use crate::core_types::webviews::editor_state::editor_actions::EditorStateActions;

#[typeshare]
#[derive(Record, Serialize, Deserialize)]
pub struct SetParsedMdxContentEditorAction {
    pub r#type: EditorStateActions,
    /// The serialized flatbuffer for the OnParsedContentChangeEventBuffer table.
    pub payload: Vec<u8>,
}

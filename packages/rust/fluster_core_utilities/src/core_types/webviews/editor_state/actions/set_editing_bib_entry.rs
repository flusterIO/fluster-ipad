use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::editor_actions::EditorStateActions;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetEditingBibEntryPayload {
    pub content: String,
    pub entry_id: String,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetEditingBibEntryAction {
    pub r#type: EditorStateActions,
    pub payload: SetEditingBibEntryPayload,
}

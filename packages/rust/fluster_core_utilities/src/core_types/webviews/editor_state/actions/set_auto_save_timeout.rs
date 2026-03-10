use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::editor_actions::EditorStateActions;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetAutoSaveTimeoutPayload {
    auto_save_timeout: u32,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetAutoSaveTimeoutAction {
    pub r#type: EditorStateActions,
    pub payload: SetAutoSaveTimeoutPayload,
}

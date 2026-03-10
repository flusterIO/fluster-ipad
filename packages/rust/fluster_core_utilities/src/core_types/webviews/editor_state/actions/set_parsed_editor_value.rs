use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::editor_actions::EditorStateActions;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetParsedValuePayload {
    value: String,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetParsedValueAction {
    pub r#type: EditorStateActions,
    pub payload: SetParsedValuePayload,
}

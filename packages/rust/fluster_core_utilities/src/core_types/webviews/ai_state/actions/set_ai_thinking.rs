use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::{
    ai_state::actions::ai_action_ids::AiAction, editor_state::editor_actions::EditorStateActions,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetAiThinkingPayload {
    pub ai_thinking: bool,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetAiThinkingAction {
    pub r#type: AiAction,
    pub payload: SetAiThinkingPayload,
}

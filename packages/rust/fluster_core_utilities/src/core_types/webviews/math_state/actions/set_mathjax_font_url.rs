use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::ai_state::actions::ai_action_ids::AiAction;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetMathjaxFontUrlPayload {
    pub mathjax_font_url: String,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetMathjaxFontUrlAction {
    pub r#type: AiAction,
    pub payload: SetMathjaxFontUrlPayload,
}

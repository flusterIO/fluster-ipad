use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::ai_state::{
    actions::ai_action_ids::AiAction, ai_state_model::FoundationModelAccessStatus,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetFoundationModelAvailabilityPayload {
    pub foundation_model_availability: FoundationModelAccessStatus,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetFoundationModelAvailabilityAction {
    pub r#type: AiAction,
    pub payload: SetFoundationModelAvailabilityPayload,
}

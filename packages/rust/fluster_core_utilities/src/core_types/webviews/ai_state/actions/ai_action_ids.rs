use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, Serialize, Deserialize, strum_macros::Display)]
pub enum AiAction {
    #[serde(rename = "set-ai-thinking")]
    #[strum(to_string = "set-ai-thinking")]
    SetAiThinking,
    #[serde(rename = "set-foundation-model-avail")]
    #[strum(to_string = "set-foundation-model-avail")]
    SetFoundationModelAvailability,
}

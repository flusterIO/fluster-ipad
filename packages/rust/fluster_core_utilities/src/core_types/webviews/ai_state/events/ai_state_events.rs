use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, uniffi::Enum, strum_macros::Display)]
pub enum AiStateEvents {
    #[serde(rename = "send-general-ai-request-phase-2")]
    #[strum(to_string = "send-general-ai-request-phase-2")]
    SendGeneralAiRequestPhase2,
}

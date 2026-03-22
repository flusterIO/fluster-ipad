use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, strum_macros::Display, Serialize, Deserialize)]
pub enum FoundationModelAccessStatus {
    #[serde(rename = "available")]
    #[strum(to_string = "available")]
    Available,
    #[serde(rename = "not-ready")]
    #[strum(to_string = "not-ready")]
    ModelNotReady,
    #[serde(rename = "unknown-status")]
    #[strum(to_string = "unknown-status")]
    UnknownStatus,
    #[serde(rename = "device-not-elig")]
    #[strum(to_string = "device-not-elig")]
    DeviceNotEligible,
    #[serde(rename = "ai-not-enabled")]
    #[strum(to_string = "ai-not-enabled")]
    AppleIntelligenceNotEnabled,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct AiState {
    pub foundation_model_access: FoundationModelAccessStatus,
    pub ai_thinking: bool,
}

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, strum_macros::Display, Serialize, Deserialize)]
pub enum UserSetLLM {
    /// The default model varies by use case.
    #[serde(rename = "Default")]
    #[strum(to_string = "Default")]
    Default,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct GeneralAiRequestPhase2Event {
    /// The content of the request as a natural language string.
    pub user_request: String,
    /// The full_match of the component that was passed into phase1. This is
    /// required to replace the content once the AI request succeeds.
    pub full_match: String,
    /// Optionally let the user provide a model that is an enum of supported
    /// models.
    pub model: UserSetLLM,
}

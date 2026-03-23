use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct GeneralAiRequestPhase2Event {
    /// The content of the request as a natural language string.
    pub user_request: String,
    /// The full_match of the component that was passed into phase1. This is
    /// required to replace the content once the AI request succeeds.
    pub full_match: String,
}

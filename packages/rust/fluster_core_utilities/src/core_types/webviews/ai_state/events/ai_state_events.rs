use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, uniffi::Enum, strum_macros::Display)]
pub enum AiStateEvents {
    #[serde(rename = "send-general-ai-request-phase-2")]
    #[strum(to_string = "send-general-ai-request-phase-2")]
    SendGeneralAiRequestPhase2,
    #[serde(rename = "stream-generated-summary")]
    #[strum(to_string = "stream-generated-summary")]
    StreamGeneratedSummary,
    #[serde(rename = "generated-summary-acceptance")]
    #[strum(to_string = "generated-summary-acceptance")]
    SetGeneratedSummaryAcceptance,
}

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, strum_macros::Display, Serialize, Deserialize)]
pub enum AiNoteInteractionType {
    #[serde(rename = "generate-summary")]
    #[strum(to_string = "generate-summary")]
    GenerateSummary,
}

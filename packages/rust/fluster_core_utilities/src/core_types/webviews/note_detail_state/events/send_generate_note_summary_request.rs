use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct GenerateNoteSummaryRequest {
    pub note_id: String,
}

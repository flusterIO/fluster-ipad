use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::note_detail_state::note_detail_state_model::SummaryGenerationMethod;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct GenerateNoteSummaryRequest {
    pub note_id: String,
    pub generation_method: SummaryGenerationMethod,
}

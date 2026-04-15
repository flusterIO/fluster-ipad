use serde::Serialize;

#[typeshare::typeshare]
#[derive(uniffi::Record, Serialize)]
pub struct StreamGeneratedSummaryAction {
    pub note_id: String,
    pub content: String,
}

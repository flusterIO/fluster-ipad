use serde::Serialize;

#[typeshare::typeshare]
#[derive(uniffi::Record, Serialize)]
pub struct SendGeneratedSummaryUserResponse {
    pub note_id: String,
    // Send the string back to Swift so that we know for sure that the state the user accepted
    // is the state that they saw.
    pub content: String,
    pub accepted: bool,
}

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct ManualSaveRequestEvent {
    pub current_note_content: String,
    /// note_id is required to verify the note before updating the note's data since there's so
    /// much async shit going on.
    pub note_id: String,
}

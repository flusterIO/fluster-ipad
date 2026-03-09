use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct EditorChangeEvent {
    pub note_id: String,
    /// The _unparsed_ content.
    pub content: String,
}

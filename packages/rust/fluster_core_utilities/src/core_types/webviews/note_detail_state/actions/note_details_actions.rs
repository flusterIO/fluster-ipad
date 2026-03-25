use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, Serialize, Deserialize, strum_macros::Display)]
pub enum NoteDetailActions {
    #[serde(rename = "set-note-details")]
    #[strum(to_string = "set-note-details")]
    SetNoteDetails,
    #[serde(rename = "invalidate-note-details")]
    #[strum(to_string = "invalidate-note-details")]
    InvalidateNoteDetails,
    #[serde(rename = "set-note-summary")]
    #[strum(to_string = "set-note-summary")]
    SetNoteSummary,
}

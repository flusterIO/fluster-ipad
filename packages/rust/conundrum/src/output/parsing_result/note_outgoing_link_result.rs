use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct NoteOutgoingLinkResult {
    /// The user defined id on the target note.
    pub link_to_note_id: String,
}

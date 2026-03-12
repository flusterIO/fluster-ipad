use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::editor_state::{EditorCitation, EditorTag};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct NoteDetailState {
    pub note_id: String,
    pub title: String,
    pub summary: Option<String>,
    pub topic: Option<String>,
    pub subject: Option<String>,
    pub tags: Vec<EditorTag>,
    pub citations: Vec<EditorCitation>,
    pub last_modified_string: String,
    pub last_read_string: String,
}

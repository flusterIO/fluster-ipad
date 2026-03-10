use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::editor_actions::EditorStateActions;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct NoteTocHeadingRustMirror {
    pub content: String,
    pub id: String,
    pub depth: u8,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct EchoNoteTocPayload {
    headings: Vec<NoteTocHeadingRustMirror>,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct EchoNoteTocEvent {
    pub r#type: EditorStateActions,
    pub payload: EchoNoteTocPayload,
}

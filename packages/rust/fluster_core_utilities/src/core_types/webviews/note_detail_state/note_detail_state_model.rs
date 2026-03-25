use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::editor_state::{EditorCitation, EditorTag};

#[typeshare]
#[derive(uniffi::Enum, Serialize, Deserialize, strum_macros::Display)]
pub enum SummaryGenerationMethod {
    #[serde(rename = "ai")]
    #[strum(to_string = "ai")]
    AI,
    #[serde(rename = "ai-manual")]
    #[strum(to_string = "ai-manual")]
    AIManualTrigger,
    #[serde(rename = "frontmatter")]
    #[strum(to_string = "frontmatter")]
    FrontMatter,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SummaryState {
    pub content: String,
    /// The javascript/unix timestamp in milliseconds.
    pub ctime: u32,
    pub generation_method: SummaryGenerationMethod,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct NoteDetailState {
    pub note_id: String,
    pub title: String,
    pub summary: Option<SummaryState>,
    pub topic: Option<String>,
    pub subject: Option<String>,
    pub tags: Vec<EditorTag>,
    pub citations: Vec<EditorCitation>,
    pub last_modified_string: String,
    pub last_read_string: String,
}

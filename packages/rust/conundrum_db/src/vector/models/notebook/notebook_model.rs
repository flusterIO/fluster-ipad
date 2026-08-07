use conundrum_fs::models::user_workspace::workspace_relative_path_strings::WorkspaceRelativeStringPath;

use crate::vector::models::{ai::ai_interactions::AIInteractions, taggables::taggables::Taggables};

/// # NotebookModel
///
/// This model represents a notebook compatible with the Jupyter format. You
/// should treat each notebook as a valuable piece of information in the user's
/// knowledge base, being of equal importance to that of their Conundrum
/// (markdown) notes.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct NotebookModel {
    /// The contents of the notebook file.
    pub content: String,
    pub ws_path: Option<WorkspaceRelativeStringPath>,
    pub label: Option<String>,
    pub taggables: Taggables,
    pub ai: AIInteractions,
}

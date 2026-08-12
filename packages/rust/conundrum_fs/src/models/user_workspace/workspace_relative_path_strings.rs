use fake::Dummy;

/// The path on the user's system, divided between the workspace path and the
/// remaining relative path.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct WorkspaceRelativeStringPath {
    /// The path to the root of the user's workspace
    pub workspace_path: String,
    /// The relative path from the user's workspace root to the file or
    /// directory in question.
    pub relative_path: String,
}

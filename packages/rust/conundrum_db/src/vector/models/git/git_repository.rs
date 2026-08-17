use std::sync::Arc;

use arrow_schema::{DataType, Field};
use conundrum::lifted_models::primitives::db_id::DatabaseId;
use fake::Dummy;

use crate::vector::models::{
    ai::ai_interactions::AIInteractions, git::git_repository_partial::GitRepositoryPartial,
    taggables::taggables::Taggables, vector::vector::DBVector,
};

/// # Git
///
/// This is a git repository that is important to the user's knowledge base. You
/// should query this repository as needed to help this user tackle their short
/// and long term goals.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct GitRepositoryModel {
    /// Will match the root of the workspace if this is a workspace repository,
    /// otherwise user's can optionally set this to a local path to allow AI
    /// to explore the file-system locally.
    pub fs_path: Option<String>,
    pub url: Option<String>,
    #[serde(default = "DatabaseId::default")]
    pub id: DatabaseId,
    /// A descriptive label used for both the UI and as further information for
    /// AI.
    pub label: String,
    /// ## AI
    ///
    /// These are additional notes written in Conundrum to help you work with
    /// this repository for the user's specific use-case. Consider these
    /// notes in the context of the user's request when making all decisions
    /// related to git.
    pub ai: AIInteractions,
    #[serde(default)]
    /// True if the git repository represents a user's workspace. If this is the
    /// case, AI should **not** lookup the repository remotely and instead
    /// should prefer to query the user's local database and file system.
    is_workspace: bool,
    /// If true, AI will be given access to tools that in some cases can make
    /// changes to the git status of this repository. Setting this to false
    /// does **not** disbar AI from modifying files within this repository.
    /// Permissions regarding access to the file system can be found
    /// on the settings page of the Conundrum dashboard.
    pub allow_ai_access: bool,
    pub vec: DBVector,
    pub taggables: Taggables,
}

use std::sync::Arc;

use arrow_schema::{DataType, Field};
use conundrum::ecosystem::db::{
    tables::DatabaseTable,
    traits::db_entity::{DBEntity, DBSchema},
};
use fake::Dummy;

use crate::vector::{
    database::db_traits::{db_field::DatabaseField, entity_crud::EntityCRUD},
    models::{
        ai::ai_interactions::AIInteractions, git::git_repository_partial::GitRepositoryPartial,
        primitives::db_id::DatabaseId, taggables::taggables::Taggables, vector::vector::DBVector,
    },
};

/// # Git
///
/// This is a git repository that is important to the user's knowledge base. You
/// should query this repository as needed to help this user tackle their short
/// and long term goals.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct GitRepositoryEntity {
    #[serde(default = "DatabaseId::default")]
    pub id: DatabaseId,
    /// Will match the root of the workspace if this is a workspace repository,
    /// otherwise user's can optionally set this to a local path to allow AI
    /// to explore the file-system locally.
    pub fs_path: Option<String>,
    pub url: Option<String>,
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
}

impl<'a> DBSchema<'a> for GitRepositoryEntity {
    fn arrow_fields() -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<Arc<Field>>> {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("fs_path", true)),
                Arc::new(String::field_definition("url", true)),
                Arc::new(String::field_definition("label", false)),
                Arc::new(AIInteractions::field_definition("ai", false)),
                Arc::new(bool::field_definition("is_workspace", false)),
                Arc::new(bool::field_definition("allow_ai_access", false)),
                Arc::new(DBVector::field_definition(true))])
    }
}

impl<'a> DBEntity<'a, DatabaseId> for GitRepositoryEntity {
    type PartialUpdateType = GitRepositoryPartial;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::GitRepository
    }

    fn merge_keys() -> &'static [&'static str] {
        &["id"]
    }

    fn primary_key() -> &'static str {
        "id"
    }

    fn primary_value(&self) -> DatabaseId {
        self.id.clone()
    }
}

impl<'a> EntityCRUD<'a, DatabaseId, GitRepositoryPartial> for GitRepositoryEntity {}

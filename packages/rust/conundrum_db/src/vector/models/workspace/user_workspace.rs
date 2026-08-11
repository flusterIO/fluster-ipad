use crate::vector::database::db_traits::db_field::DatabaseField;
use crate::vector::database::db_traits::entity_crud::EntityCRUD;
use crate::vector::database::db_traits::validate::ValidateSelf;
use crate::vector::models::ai::ai_interactions::AIInteractions;
use crate::vector::models::workspace::user_workspace_partial::UserWorkspacePartial;
use conundrum::ecosystem::db::traits::db_entity::DBEntity;
use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use conundrum::ecosystem::error_handling::db_error::DatabaseResult;
use conundrum::lang::constants::file_types::ParsableFileType;
use conundrum_fs::workspace_management::file_walk_config::FileWalkConfig;
use conundrum_fs::workspace_management::get_filetype_recursively::get_filetype_in_workspace_recursively;
use fake::Dummy;
use futures_util::TryStreamExt;
use lancedb::arrow::arrow_schema::DataType;
use lancedb::arrow::arrow_schema::Field;
use lancedb::query::ExecutableQuery;
use lancedb::query::QueryBase;
use serde::{Deserialize, Serialize};
use serde_arrow::from_record_batch;
use specta::Type;
use std::path::{Path, PathBuf};
use std::sync::Arc;

static USER_WORKSPACE_PRIMARY_KEY: &str = "root";
static USER_WORKSPACE_MERGE_KEYS: &[&str] = &[USER_WORKSPACE_PRIMARY_KEY];

#[derive(Serialize, Deserialize, Clone, Debug, Type, Dummy)]
pub struct UserWorkspace {
    /// The path to the root of the workspace and the primary key for the
    /// workspace.
    pub root: String,
    /// A short, descriptive label for this workspace.
    pub label: Option<String>,
    /// Many Conundrum search methods will ignore files based on any
    /// `.gitignore` files found within the user's workspace.
    pub respect_gitignore: bool,
    /// Ignore files hidden by the user's operating system.
    pub ignore_hidden: bool,
    /// A directory that can be used as a shortcut within url strings when
    /// loading media, making paths relative to this directory valid.
    /// ### Example
    /// ```
    /// <Image src="physics/images/recent_plot.png" />
    /// ```
    /// Where `physics/iamges/recent_plot.png` is a path nested within the
    /// `resource_dir` directory.
    #[serde(default = "Default::default")]
    pub resource_dir: String,
    pub ai: AIInteractions,
}

impl<'a> DBSchema<'a> for UserWorkspace {
    fn arrow_fields() -> DatabaseResult<Vec<Arc<Field>>> {
        Ok(vec![Arc::new(String::field_definition("root", false)),
                Arc::new(String::field_definition("label", true)),
                Arc::new(bool::field_definition("respect_gitignore", false)),
                Arc::new(bool::field_definition("ignore_hidden", false)),
                Arc::new(String::field_definition("resource_dir", false)),
                Arc::new(AIInteractions::field_definition("ai", false))])
    }
}

impl UserWorkspace {
    pub async fn exists(&self) -> bool {
        tokio::fs::try_exists(self.root.clone()).await.is_ok_and(|x| x)
    }

    pub fn join_path<P: AsRef<Path>>(&self, fp: P) -> PathBuf {
        Path::new(&self.root).join(fp)
    }

    pub async fn parsable_files_of_type(&self, parsable_file_type: ParsableFileType) -> DatabaseResult<Vec<String>> {
        let c = self.into_file_walk_config(parsable_file_type);
        let mutex = get_filetype_in_workspace_recursively(c).await.map_err(DatabaseError::FileSystemError)?;
        if let Ok(items) = Arc::try_unwrap(mutex) {
            let x = items.into_inner();
            Ok(x)
        } else {
            Err(DatabaseError::ThreadError)
        }
    }

    pub fn from_root_and_label(root: String, label: Option<String>) -> Self {
        Self { root,
               label,
               ignore_hidden: true,
               respect_gitignore: true,
               resource_dir: "/resources".to_string(),
               ai: AIInteractions::default() }
    }

    pub fn item_field_def() -> Field {
        Field::new("item", DataType::Utf8, true)
    }

    fn into_file_walk_config(&self, file_type: ParsableFileType) -> FileWalkConfig {
        FileWalkConfig { ignore_hidden: self.ignore_hidden,
                         respect_git_ignore: self.respect_gitignore,
                         root: self.root.clone(),
                         file_type }
    }
}

impl ValidateSelf for UserWorkspace {
    async fn validate(&self) -> DatabaseResult<()> {
        if self.exists().await {
            Ok(())
        } else {
            Err(DatabaseError::InvalidWorkspacePath(self.root.clone()))
        }
    }
}

impl From<String> for UserWorkspace {
    fn from(value: String) -> Self {
        Self { root: value,
               label: None,
               respect_gitignore: true,
               ignore_hidden: true,
               resource_dir: "/resources".to_string(),
               ai: AIInteractions::default() }
    }
}

impl<'a> EntityCRUD<'a, String, UserWorkspacePartial> for UserWorkspace {
    async fn get_by_predicate(predicate: Option<String>,
                              pagination: Option<crate::vector::database::pagination::PaginationParams>,
                              db: &crate::vector::database::db::ArcMutexDB)
                              -> DatabaseResult<Vec<Self>>
        where Self: Sized {
        {
            let _db = db.clone().lock_owned().await;
            let self_table = Self::table();
            let tbl = crate::vector::database::open_table::open_table(_db, &self_table).await?;
            let mut query_builder = tbl.query();
            if let Some(_predicate) = predicate.clone() {
                query_builder = query_builder.only_if(_predicate);
            }
            if let Some(_pagination) = pagination {
                let (limit, offset) = _pagination.to_limit_and_offset();
                query_builder = query_builder.limit(limit).offset(offset);
            }
            let res = query_builder.execute()
                                   .await
                                   .map_err(|e| {
                                       log::error!("Error: {:?}", e);
                                       DatabaseError::FailToQueryEntity { predicate: predicate.clone(),
                                                                          table: self_table.clone() }
                                   })?
                                   .try_collect::<Vec<_>>()
                                   .await
                                   .map_err(|e| {
                                       log::error!("Error: {:?}", e);
                                       DatabaseError::SerializationError
                                   })?;
            if res.is_empty() {
                return Ok(Vec::new());
            }
            let mut items: Vec<UserWorkspace> = Vec::new();
            for record_batch in res.iter() {
                let r: Vec<UserWorkspace> = from_record_batch(record_batch).map_err(|e| {
                                                                               log::error!("Error: {:?}", e);
                                                                               DatabaseError::SerializationError
                                                                           })?;
                items.extend(r);
            }
            Ok(items)
        }
    }
}

impl<'a> DBEntity<'a> for UserWorkspace {
    type PartialUpdateType = UserWorkspacePartial;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::UserWorkspace
    }

    fn merge_keys() -> &'static [&'static str] {
        USER_WORKSPACE_MERGE_KEYS
    }

    fn primary_key() -> &'static str {
        USER_WORKSPACE_PRIMARY_KEY
    }

    fn primary_value(&self) -> String {
        self.root.clone()
    }
}

#[cfg(test)]
mod tests {
    use fake::Fake;

    use crate::test_crud_functionality;

    use super::*;

    #[tokio::test]
    async fn user_workspace_crud_functionality() {
        test_crud_functionality!(UserWorkspace, "UserWorkspace")
    }
}

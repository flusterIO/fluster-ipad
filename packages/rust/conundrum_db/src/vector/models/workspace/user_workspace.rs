use std::collections::HashMap;
use std::path::Path;
use std::{path::PathBuf, sync::Arc};

use conundrum::ecosystem::error_handling::conundrum_fs_error::ConundrumFSResult;
use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use conundrum::ecosystem::error_handling::db_error::DatabaseResult;
use fake::Dummy;
use lancedb::arrow::arrow_schema::DataType;
use lancedb::arrow::arrow_schema::Field;
use serde::{Deserialize, Serialize};
use serde_arrow::to_record_batch;
use specta::Type;

use crate::vector::database::db_traits::db_entity::ArrowSchemaRepresentable;
use crate::vector::database::db_traits::entity_crud::EntityCRUD;
use crate::vector::database::db_traits::{db_entity::DBEntity, validate::ValidateSelf};
use crate::vector::models::workspace::user_workspace_partial::UserWorkspacePartial;

static USER_WORKSPACE_PRIMARY_KEY: &str = "root";
static USER_WORKSPACE_MERGE_KEYS: &[&str] = &[USER_WORKSPACE_PRIMARY_KEY];

fn default_bib_path() -> Vec<String> {
    vec![String::from("/citations.bib")]
}

#[derive(Serialize, Deserialize, Clone, Debug, Type, Dummy)]
pub struct UserWorkspace {
    /// The path to the root of the workspace and the primary key for the
    /// workspace.
    pub root: String,
    pub label: Option<String>,
    pub respect_gitignore: bool,
    pub ignore_hidden: bool,
    #[serde(default = "default_bib_path")]
    pub bib_paths: Vec<String>,
    #[serde(default = "Default::default")]
    pub resource_dir: String,
}

impl UserWorkspace {
    pub async fn exists(&self) -> bool {
        tokio::fs::try_exists(self.root.clone()).await.is_ok_and(|x| x)
    }

    pub fn join_path<P: AsRef<Path>>(&self, fp: P) -> PathBuf {
        Path::new(&self.root).join(fp)
    }

    pub async fn valid_bib_paths(&self) -> ConundrumFSResult<HashMap<String, bool>> {
        if self.bib_paths.is_empty() {
            let hm: HashMap<String, bool> = HashMap::new();
            Ok(hm)
        } else {
            let mut exists_data: HashMap<String, bool> = HashMap::new();
            for bp in self.bib_paths.clone() {
                let abs_path = self.join_path(&bp);
                let exists = tokio::fs::try_exists(abs_path).await.is_ok_and(|x| x);
                exists_data.insert(bp.to_string(), exists);
            }
            Ok(exists_data)
        }
    }

    pub fn from_root_and_label(root: String, label: Option<String>) -> Self {
        Self { root,
               label,
               bib_paths: default_bib_path(),
               ignore_hidden: true,
               respect_gitignore: true,
               resource_dir: "/resources".to_string() }
    }

    pub fn item_field_def() -> Field {
        Field::new("item", DataType::Utf8, true)
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
               bib_paths: default_bib_path(),
               respect_gitignore: true,
               ignore_hidden: true,
               resource_dir: "/resources".to_string() }
    }
}

impl ArrowSchemaRepresentable for UserWorkspace {
    fn arrow_schema() -> std::sync::Arc<arrow_schema::Schema> {
        Arc::new(arrow_schema::Schema::new(vec![Field::new("root", DataType::Utf8, false),
                                                Field::new("label", DataType::Utf8, true),
                                                Field::new("bib_paths",
                                                           DataType::List(Arc::new(Self::item_field_def())),
                                                           true),
                                                Field::new("respect_gitignore", DataType::Boolean, false),
                                                Field::new("ignore_hidden", DataType::Boolean, false),
                                                Field::new("resource_dir", DataType::Utf8, true),]))
    }
}

impl DBEntity for UserWorkspace {
    type PartialUpdateType = UserWorkspacePartial;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::UserWorkspace
    }

    fn get_record_batch(data: Vec<Self>)
                        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<arrow_array::RecordBatch>
        where Self: Sized {
        to_record_batch(&Self::arrow_schema().fields, &data.clone()).map_err(|e| {
                                                                        log::error!("Error: {:?}", e);
                                                                        DatabaseError::SerializationError
                                                                    })
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

impl EntityCRUD<String, UserWorkspacePartial> for UserWorkspace {}

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

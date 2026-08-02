use std::{path::PathBuf, sync::Arc};

use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use conundrum::ecosystem::error_handling::db_error::DatabaseResult;
use lancedb::arrow::arrow_schema::DataType;
use lancedb::arrow::arrow_schema::Field;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::vector::{
    database::{
        db::ArcMutexDB,
        db_traits::{db_entity::DBEntity, validate::ValidateSelf},
    },
    models::{joins::one_to_many::OneToMany, text::cdrm::cdrm_content::CdrmContent},
};

static USER_WORKSPACE_PRIMARY_KEY: &str = "value_lc";
static USER_WORKSPACE_MERGE_KEYS: &[&str] = &[USER_WORKSPACE_PRIMARY_KEY];

fn default_bib_path() -> String {
    String::from("/citations.bib")
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct UserWorkspace {
    /// The path to the root of the workspace and the primary key for the
    /// workspace.
    pub root: String,
    pub label: Option<String>,
    #[serde(default = "default_bib_path")]
    pub bib_path: String,
}

impl UserWorkspace {
    pub async fn exists(&self) -> bool {
        tokio::fs::try_exists(self.root.clone()).await.is_ok_and(|x| x)
    }

    pub fn from_root_and_label(root: String, label: Option<String>) -> Self {
        Self { root,
               label,
               bib_path: default_bib_path() }
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
               bib_path: default_bib_path() }
    }
}

impl DBEntity for UserWorkspace {
    fn arrow_schema() -> std::sync::Arc<arrow_schema::Schema> {
        Arc::new(arrow_schema::Schema::new(vec![Field::new("label", DataType::Utf8, true),
                                                Field::new("root", DataType::Utf8, false),]))
    }

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::UserWorkspace
    }

    fn get_record_batch(data: Vec<Self>)
                        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<arrow_array::RecordBatch>
        where Self: Sized {
        let mut roots: Vec<String> = Vec::new();
        let mut labels: Vec<Option<String>> = Vec::new();
        for item in data {
            roots.push(item.root.clone());
            labels.push(item.label.clone());
        }
        let roots_array = arrow_array::StringArray::from(roots);
        let labels_array = arrow_array::StringArray::from(labels);
        arrow_array::RecordBatch::try_new(Self::arrow_schema(), vec![
            Arc::new(roots_array),
            Arc::new(labels_array)
        ])
            .map_err(|e| {
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
}

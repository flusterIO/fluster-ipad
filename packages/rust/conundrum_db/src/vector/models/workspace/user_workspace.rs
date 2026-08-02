use std::{path::PathBuf, sync::Arc};

use arrow_schema::DataType;
use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use fake::vec;
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

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct UserWorkspace {
    /// The path to the root of the workspace and the primary key for the
    /// workspace.
    pub root: String,
    pub label: Option<String>,
}

impl UserWorkspace {
    pub async fn exists(&self) -> bool {
        tokio::fs::try_exists(self.root).await.is_ok_and(|x| x)
    }
}

impl ValidateSelf for UserWorkspace {
    async fn validate(&self) -> conundrum::ecosystem::error_handling::db_error::DatabaseError<()> {
        if self.exists() {
            Ok(())
        } else {
            Err(DatabaseError::InvalidWorkspacePath(self.root.clone()))
        }
    }
}

impl From<String> for UserWorkspace {
    fn from(value: String) -> Self {
        Self { root: value,
               label: None }
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
        &[Self::primary_key()]
    }

    fn primary_key() -> &'static str {
        "root"
    }
}

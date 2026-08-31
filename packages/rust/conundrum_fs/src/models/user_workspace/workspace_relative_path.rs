use std::path::PathBuf;

use arrow_schema::{Field, Fields};
use conundrum::ecosystem::{
    db::db_traits::db_field::DatabaseField, error_handling::conundrum_fs_error::ConundrumFSResult,
};
use serde::{Deserialize, Serialize};

use crate::models::user_workspace::cdrm_path_buf::CDRMPathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkspaceRelativePath<T = CDRMPathBuf> {
    pub workspace_path: T,
    pub relative_path: T,
}

impl WorkspaceRelativePath<PathBuf> {
    pub fn absolutize(&self) -> PathBuf {
        self.workspace_path.join(self.relative_path.clone())
    }

    pub fn from_path_and_root(absolute_path: String, root: String) -> ConundrumFSResult<Self> {
        if let Some(relative_path) = pathdiff::diff_paths(absolute_path, root.clone()) {
            Ok(WorkspaceRelativePath { workspace_path: PathBuf::new().join(root),
                                       relative_path })
        } else {
            log::warn!("Attempted to get a nested path from a parent that isn't that path's parent.");
            Err(conundrum::ecosystem::error_handling::conundrum_fs_error::ConundrumFSError::PathSerializationError)
        }
    }
}

impl DatabaseField for WorkspaceRelativePath {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field {
        Field::new(field_key.to_string(),
                   arrow_schema::DataType::Struct(Fields::from(vec![Field::new("workspace_path",
                                                                               arrow_schema::DataType::Utf8,
                                                                               false),
                                                                    Field::new("relative_path",
                                                                               arrow_schema::DataType::Utf8,
                                                                               false),])),
                   nullable)
    }
}

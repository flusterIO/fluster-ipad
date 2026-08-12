use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use conundrum_fs::models::user_workspace::workspace_relative_path_strings::WorkspaceRelativeStringPath;
use fake::Dummy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::vector::{
    database::db_traits::{
        db_field::{DatabaseField, DatabaseFieldLarge},
        impls::workspace_relative_path_field::workspace_relative_path_field,
    },
    models::{
        ai::ai_interactions::AIInteractions,
        date_time::date_time::DateTime,
        primitives::{bytes::Bytes, db_id::DatabaseId},
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct BinaryFileModel {
    pub id: DatabaseId,
    pub ws_path: Option<WorkspaceRelativeStringPath>,
    pub data: Bytes,
    pub ai: AIInteractions,
    pub ctime: DateTime,
    pub utime: DateTime,
}

impl<'a> DBSchema<'a> for BinaryFileModel {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(workspace_relative_path_field("ws_path", true)),
                Arc::new(Bytes::field_definition_large("data", false)),
                Arc::new(AIInteractions::field_definition("ai", false)),
                Arc::new(DateTime::field_definition("ctime", false)),
                Arc::new(DateTime::field_definition("utime", false)),])
    }
}

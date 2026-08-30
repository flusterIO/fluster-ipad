use conundrum::{
    ecosystem::db::db_traits::{
        db_entity::DBSchema,
        db_field::{DatabaseField, DatabaseFieldLarge},
        impls::workspace_relative_path_field::workspace_relative_path_field,
    },
    lifted_models::primitives::{bytes::Bytes, date_time::DateTime, db_id::DatabaseId},
};
use conundrum_fs::models::user_workspace::workspace_relative_path_strings::WorkspaceRelativeStringPath;
use conundrum_macros::{DBSchema, DatabaseEntity};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::vector::models::ai::ai_interactions::AIInteractions;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, DBSchema)]
pub struct BinaryFileModel {
    pub id: DatabaseId,
    pub ws_path: Option<WorkspaceRelativeStringPath>,
    pub data: Bytes,
    pub ai: AIInteractions,
    pub ctime: DateTime,
    pub utime: DateTime,
}

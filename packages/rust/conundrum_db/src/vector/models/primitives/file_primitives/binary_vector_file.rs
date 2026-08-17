use conundrum::lifted_models::primitives::{bytes::Bytes, db_id::DatabaseId};
use conundrum_fs::models::user_workspace::workspace_relative_path_strings::WorkspaceRelativeStringPath;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{ai::ai_interactions::AIInteractions, vector::vector::DBVector};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct BinaryVectorFileModel {
    pub id: DatabaseId,
    pub ws_path: Option<WorkspaceRelativeStringPath>,
    pub data: Bytes,
    pub ai: AIInteractions,
    pub vector: DBVector,
}

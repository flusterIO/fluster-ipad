use crate::vector::models::primitives::db_id::DatabaseId;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct UserWorkspaceRepository {
    pub workspace_root: String,
    pub repository_id: DatabaseId,
}

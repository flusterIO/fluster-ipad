use crate::lifted_models::primitives::db_id::DatabaseId;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type, fake::Dummy)]
pub struct ChatHistoryParams {
    pub convo_id: DatabaseId,
    pub max_count: u32,
}

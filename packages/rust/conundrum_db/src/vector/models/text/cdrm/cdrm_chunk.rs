use conundrum::lifted_models::primitives::db_id::DatabaseId;
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy)]
pub struct CdrmChunk {
    pub content: String,
    pub note_id: DatabaseId,
    pub chunk_idx: String,
    pub vec: Vec<f64>,
}

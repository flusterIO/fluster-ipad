use serde::{Deserialize, Serialize};

use crate::vector::models::primitives::db_id::DatabaseId;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CdrmChunk {
    pub content: String,
    pub note_id: DatabaseId,
    pub chunk_idx: String,
    pub vec: Vec<f64>,
}

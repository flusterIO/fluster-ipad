use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

use crate::vector::models::primitives::db_id::DatabaseId;

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct CdrmChunk {
    pub content: String,
    pub note_id: DatabaseId,
    pub chunk_idx: String,
    pub vec: Vec<f64>,
}

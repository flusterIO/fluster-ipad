use serde::{Deserialize, Serialize};

use crate::vector::models::primitives::db_id::DatabaseId;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TypstChunk {
    pub content: String,
    /// The id of the primary typst content that provided this chunk.
    pub typst_content_id: DatabaseId,
    pub chunk_idx: String,
    pub vec: Vec<f64>,
}

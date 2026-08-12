use crate::vector::models::{primitives::db_id::DatabaseId, vector::vector::DBVector};

pub struct PdfChunk {
    pub pdf_id: DatabaseId,
    pub embedding_text: String,
    pub vector: DBVector,
}

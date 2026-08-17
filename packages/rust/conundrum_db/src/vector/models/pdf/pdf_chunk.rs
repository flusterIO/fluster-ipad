use conundrum::lifted_models::primitives::db_id::DatabaseId;

use crate::vector::models::vector::vector::DBVector;

pub struct PdfChunk {
    pub pdf_id: DatabaseId,
    pub embedding_text: String,
    pub vector: DBVector,
}

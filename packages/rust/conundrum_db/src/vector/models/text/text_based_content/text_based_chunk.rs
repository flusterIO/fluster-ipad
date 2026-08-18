use conundrum::{
    ai::rig::ai_traits::into_embedding_description::IntoEmbeddingDescription,
    ecosystem::db::{
        db_traits::{
            db_entity::{DBEntity, DBSchema},
            db_field::DatabaseField,
        },
        tables::DatabaseTable,
    },
    impl_default_crud,
    lifted_models::primitives::db_id::DatabaseId,
};
use rig::Embed;
use std::sync::Arc;

use crate::vector::{models::vector::vector::DBVector, parameters};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, fake::Dummy)]
pub struct TextBasedChunk {
    pub id: DatabaseId,
    /// The id pointing back to the primary document. There's no point in having
    /// an actual id on a struct that will always retrieved by vector
    /// similarity.
    pub document_id: DatabaseId,
    pub content: String,
    /// The index of the chunk as it appears in the whole document.
    pub chunk_idx: u32,
    pub vector: DBVector,
}

impl<'a> DBSchema<'a> for TextBasedChunk {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(DatabaseId::field_definition("document_id", false)),
                Arc::new(String::field_definition("content", false)),
                Arc::new(u32::field_definition("chunk_idx", false)),
                Arc::new(DBVector::field_definition(false)),])
    }
}

impl<'a> DBEntity<'a, DatabaseId> for TextBasedChunk {
    type PartialUpdateType = TextBasedChunk;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::DocumentationChunk
    }

    fn merge_keys() -> &'static [&'static str] {
        &["id"]
    }

    fn primary_key() -> &'static str {
        "document_id"
    }

    fn primary_value(&self) -> DatabaseId {
        self.document_id.clone()
    }
}

impl Embed for TextBasedChunk {
    fn embed(&self, embedder: &mut rig::embeddings::TextEmbedder) -> Result<(), rig::embeddings::EmbedError> {
        embedder.embed(self.content.clone());
        Ok(())
    }
}

impl IntoEmbeddingDescription for TextBasedChunk {
    fn into_embedding_description(&self) -> String {
        self.content.clone()
    }

    fn human_readable_model_name() -> &'static str {
        "TextBasedChunk"
    }
}

impl_default_crud!(TextBasedChunk, TextBasedChunk, DatabaseId);

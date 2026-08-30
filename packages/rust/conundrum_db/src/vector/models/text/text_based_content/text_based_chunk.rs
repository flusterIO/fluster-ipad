use conundrum::{
    ai::{
        models::chat::vector::vector_model::DBVector,
        rig::ai_traits::into_embedding_description::IntoEmbeddingDescription,
    },
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
use conundrum_macros::{DBEntity, DBSchema};
use rig::Embed;
use std::sync::Arc;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, fake::Dummy, specta::Type, DBSchema)]
pub struct TextBasedChunk {
    pub id: DatabaseId,
    /// The id pointing back to the primary document. There's no point in having
    /// an actual id on a struct that will always retrieved by vector
    /// similarity.
    pub document_id: DatabaseId,
    pub content: String,
    /// The index of the chunk as it appears in the whole document.
    pub chunk_idx: u32,
    pub local_vector: DBVector,
    pub remote_vector: Option<DBVector>,
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

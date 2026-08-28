use crate::{
    ai::models::{chat::chat_message::any_message::AnyChatMessage, memory::memory_model::Memory},
    ecosystem::error_handling::db_error::DatabaseResult,
    lifted_models::primitives::db_id::DatabaseId,
};

pub trait ConversationStore {
    async fn append(&self, message: AnyChatMessage) -> DatabaseResult<()>;

    async fn history(&self, conversation_id: DatabaseId) -> DatabaseResult<Vec<AnyChatMessage>>;
}

pub trait MemoryStore {
    async fn remember(&self, memory: Memory) -> DatabaseResult<()>;

    async fn search(&self, query: &str, limit: usize) -> DatabaseResult<Vec<Memory>>;
}

use crate::{
    ai::models::{chat::chat_message::chat_context_policy::ChatContextPolicy, memory::memory_model::Memory},
    ecosystem::{db::db::ArcMutexDB, error_handling::db_error::DatabaseResult},
    lifted_models::primitives::db_id::DatabaseId,
};

pub trait ConversationStore<AnyChatMessageType> {
    async fn append(&mut self, message: AnyChatMessageType, db: ArcMutexDB) -> DatabaseResult<()>;

    async fn history(&self,
                     conversation_id: DatabaseId,
                     policy: ChatContextPolicy,
                     db: ArcMutexDB)
                     -> DatabaseResult<Vec<AnyChatMessageType>>;
}

pub trait MemoryStore {
    async fn remember(&self, memory: Memory, db: ArcMutexDB) -> DatabaseResult<()>;

    async fn search(&self, query: &str, limit: usize, db: ArcMutexDB) -> DatabaseResult<Vec<Memory>>;
}

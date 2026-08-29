use crate::{
    ai::models::{
        chat::chat_message::{any_message::AnyChatMessage, chat_context_policy::ChatContextPolicy},
        memory::memory_model::Memory,
    },
    ecosystem::{db::db::ArcMutexDB, error_handling::db_error::DatabaseResult},
    lifted_models::primitives::db_id::DatabaseId,
};

pub trait ConversationStore {
    async fn append(&mut self, message: AnyChatMessage, db: ArcMutexDB) -> DatabaseResult<()>;

    async fn history(&self,
                     conversation_id: DatabaseId,
                     policy: ChatContextPolicy,
                     db: ArcMutexDB)
                     -> DatabaseResult<Vec<AnyChatMessage>>;
}

pub trait MemoryStore {
    async fn remember(&self, memory: Memory, db: ArcMutexDB) -> DatabaseResult<()>;

    async fn search(&self, query: &str, limit: usize, db: ArcMutexDB) -> DatabaseResult<Vec<Memory>>;
}

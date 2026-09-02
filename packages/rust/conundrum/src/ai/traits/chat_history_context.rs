use crate::{
    ai::models::{chat::chat_message::chat_context_policy::ChatContextPolicy, memory::memory_model::Memory},
    ecosystem::{db::db_client::db_client::DBClient, error_handling::db_error::DatabaseResult},
    lifted_models::primitives::db_id::DatabaseId,
};

pub trait ConversationStore<AnyChatMessageType> {
    async fn append(&mut self, message: AnyChatMessageType, db: DBClient) -> DatabaseResult<()>;

    async fn history(&self,
                     conversation_id: DatabaseId,
                     policy: ChatContextPolicy,
                     db: DBClient)
                     -> DatabaseResult<Vec<AnyChatMessageType>>;
}

pub trait MemoryStore {
    async fn remember(&self, memory: Memory, db: DBClient) -> DatabaseResult<()>;

    async fn search(&self, query: &str, limit: usize, db: DBClient) -> DatabaseResult<Vec<Memory>>;
}

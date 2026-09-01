use std::sync::Arc;

use conundrum::{
    ai::traits::chat_history_context::{ConversationStore, MemoryStore},
    ecosystem::error_handling::db_error::DatabaseResult,
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_db::vector::models::ai::any_message::AnyChatMessage;

/// ## TO-DO
///
/// - [ ] Embed message chunks in vector space.
/// - [ ] Retrieve message chunks from semantic search instead of based on
///   recency.
/// - [ ] Allow storing of permanent 'memory' chunks in a separate table
pub struct ChatHistoryContext<C, M>
    where C: ConversationStore<AnyChatMessage>,
          M: MemoryStore {
    pub conversations: Arc<C>,
    pub memories: Arc<M>,
}

impl<C, M> ChatHistoryContext<C, M>
    where C: ConversationStore<AnyChatMessage>,
          M: MemoryStore
{
    pub async fn build_context(&self,
                               conversation_id: DatabaseId,
                               user_message: &str)
                               -> DatabaseResult<ChatHistoryContext<C, M>> {
        todo!()
    }
}

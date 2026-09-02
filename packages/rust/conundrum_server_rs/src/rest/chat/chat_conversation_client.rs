use std::sync::Arc;

use conundrum::{
    ai::{
        models::chat::chat_message::chat_context_policy::ChatContextPolicy,
        traits::chat_history_context::ConversationStore,
    },
    ecosystem::db::db_client::db_client::DBClient,
};
use conundrum_db::vector::models::ai::any_message::AnyChatMessage;

pub struct ChatConversationClient {
    pub messages: Vec<AnyChatMessage>,
}

impl ConversationStore<AnyChatMessage> for ChatConversationClient {
    /// ## To-Do
    ///
    /// - [ ] Chunk messages
    /// - [ ] Embed messages (Remote)
    /// - [ ] Embed messages (Local)
    async fn append(&mut self,
                    message: AnyChatMessage,
                    db: DBClient)
                    -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<()> {
        message.save(db.clone()).await?;
        self.messages.push(message);
        Ok(())
    }

    /// ### To-Do
    ///
    /// If the user is online with remote AI available:
    ///
    /// - [ ] Embed the user's query
    /// - [ ] Use that embedding to retrieve `n` nearest message chunks
    /// - [ ] Load unique messages based on chunks.
    /// - [ ] Truncate loaded messages within policy bounds
    ///
    /// If they're not:
    ///
    /// If they have Ollama available installed:
    /// - [ ] Use the backup embeddings to perform the same vector based
    ///   approach as above
    ///
    /// If they don't:
    /// - [ ] Load messages based on recency
    /// - [ ] Truncate them within policy bounds
    async fn history(&self,
                     conversation_id: conundrum::lifted_models::primitives::db_id::DatabaseId,
                     policy: ChatContextPolicy,
                     db: DBClient)
                     -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<AnyChatMessage>> {
        todo!()
    }
}

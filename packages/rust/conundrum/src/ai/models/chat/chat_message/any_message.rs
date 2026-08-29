use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::{
    ai::models::chat::chat_message::{
        ai::ai_message::AIMessage, system::system_prompt_message::SystemPromptMessage, user::user_message::UserMessage,
    },
    ecosystem::{
        db::{db::ArcMutexDB, db_traits::entity_crud::EntityCRUD},
        error_handling::db_error::DatabaseResult,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub enum AnyChatMessage {
    User(UserMessage),
    Agent(AIMessage),
    SystemPrompt(SystemPromptMessage),
}

impl AnyChatMessage {
    pub async fn save(&self, db: ArcMutexDB) -> DatabaseResult<()> {
        match self {
            Self::SystemPrompt(s) => SystemPromptMessage::save_many(vec![s.clone()], std::sync::Arc::clone(&db)).await,
            Self::Agent(s) => AIMessage::save_many(vec![s.clone()], std::sync::Arc::clone(&db)).await,
            Self::User(s) => UserMessage::save_many(vec![s.clone()], std::sync::Arc::clone(&db)).await,
        }
    }
}

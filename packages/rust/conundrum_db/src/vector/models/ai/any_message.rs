use fake::Dummy;
use serde::{Deserialize, Serialize};

use conundrum::{
    ai::{
        models::chat::chat_message::{
            ai::ai_message::AIMessage, chat_message_sender::ChatMessageSender, message_chunk::MessageChunk,
            system::system_prompt_message::SystemPromptMessage, user::user_message::UserMessage,
        },
        rig::ai_traits::chunk::Chunk,
    },
    ecosystem::{
        db::{db::ArcMutexDB, db_traits::entity_crud::EntityCRUD},
        error_handling::{ai_error::AIResult, db_error::DatabaseResult},
    },
    lang::runtime::run_conundrum::ParseConundrumOptions,
};

use crate::vector::models::ecosystem_data::server_state::server_state::ServerState;

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

impl Chunk<ParseConundrumOptions, MessageChunk, ServerState> for AnyChatMessage {
    async fn try_chunk(&self,
                       opts: ParseConundrumOptions,
                       state: &std::sync::Arc<ServerState>)
                       -> AIResult<Vec<MessageChunk>> {
        let (body, sender) = {
            match self {
                Self::User(u) => (u.body.clone(), ChatMessageSender::User),
                Self::Agent(a) => (a.body.clone(), ChatMessageSender::Agent),
                Self::SystemPrompt(s) => (s.body.clone(), ChatMessageSender::SystemPrompt),
            }
        };

        todo!()
    }
}

use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::ai::models::chat::chat_message::{
    ai::ai_message::AIMessage, system::system_prompt_message::SystemPromptMessage, user::user_message::UserMessage,
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub enum AnyChatMessage {
    User(UserMessage),
    Agent(AIMessage),
    SystemPrompt(SystemPromptMessage),
}

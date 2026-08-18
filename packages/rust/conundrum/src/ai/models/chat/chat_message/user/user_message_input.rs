use std::sync::Arc;

use crate::{
    ai::{
        models::chat::{chat_message::user::user_message::UserMessage, chat_sender::chat_sender::ChatParticipant},
        rig::ai_traits::ai_chat_history_item::IntoChatHistoryItem,
    },
    ecosystem::db::db_traits::{
        db_entity::{DBEntity, DBSchema},
        db_field::DatabaseField,
    },
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use axum::extract::ws::Message;
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct UserMessageInput {
    pub conversation_id: DatabaseId,
    /// If the sender is the user, this is the agent requested. If the sender is
    /// AI, this is the AI sending the response.
    pub agent_id: DatabaseId,
    pub body: String,
}

impl Into<Message> for UserMessageInput {
    fn into(self) -> Message {
        if let Ok(s) = serde_json::to_string(&self) {
            Message::Text(s.into())
        } else {
            Message::Text("{}".into())
        }
    }
}

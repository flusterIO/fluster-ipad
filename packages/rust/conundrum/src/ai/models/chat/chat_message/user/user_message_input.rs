use crate::lifted_models::primitives::db_id::DatabaseId;
use axum::extract::ws::Message;
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct UserMessageInput {
    pub conversation_id: Option<DatabaseId>,
    /// If the sender is the user, this is the agent requested. If the sender is
    /// AI, this is the AI sending the response.
    pub agent_id: Option<DatabaseId>,
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

use crate::lifted_models::primitives::db_id::DatabaseId;
use axum::extract::ws::Message;
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct UserMessageInput {
    pub convo_id: Option<DatabaseId>,
    /// If the sender is the user, this is the agent requested. If the sender is
    /// AI, this is the AI sending the response.
    pub agent_id: Option<DatabaseId>,
    pub model: Option<ModelSelection>,
    pub body: String,
}

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct ModelSelection {
    /// Stable provider key, such as `ollama` or `openai`.
    pub provider: String,
    /// Provider-specific model identifier.
    pub model_id: String,
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

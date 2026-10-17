use crate::ecosystem::db::db_traits::db_field::DatabaseField;
use crate::lifted_models::primitives::db_id::DatabaseId;
use axum::extract::ws::Message;
use fake::Dummy;
use lancedb::arrow::arrow_schema::{DataType, Field};
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

impl DatabaseField for ModelSelection {
    fn field_definition(field_key: &'static str, nullable: bool) -> lancedb::arrow::arrow_schema::Field {
        let fields = vec![std::sync::Arc::new(Field::new("provider", DataType::Utf8, false)),
                          std::sync::Arc::new(Field::new("model_id", DataType::Utf8, false)),];
        Field::new(field_key, DataType::Struct(fields.into()), nullable)
    }
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

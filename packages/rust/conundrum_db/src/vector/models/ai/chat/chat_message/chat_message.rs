use std::sync::Arc;

use axum::{extract::ws::Message, response::IntoResponse};
use conundrum::ecosystem::db::traits::db_entity::{DBEntity, DBSchema};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::{
    impl_default_crud,
    vector::{
        database::db_traits::db_field::DatabaseField,
        models::{
            ai::chat::chat_sender::chat_sender::ChatParticipant, date_time::date_time::DateTime,
            primitives::db_id::DatabaseId,
        },
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct ChatMessage {
    pub id: DatabaseId,
    pub conversation_id: DatabaseId,
    /// If the sender is the user, this is the agent reqeusted. If the sender is
    /// AI, this is the AI sending the response.
    pub agent_id: Option<DatabaseId>,
    pub sender: ChatParticipant,
    pub body: String,
    pub ctime: DateTime,
}

// impl From<ChatMessage> for Message {
//     fn from(value: ChatMessage) -> Self {
//         if let Ok(s) = serde_json::to_string(&value) {
//             Message::Text(s.into())
//         } else {
//             Message::Text("{}".into())
//         }
//     }
// }

impl Into<Message> for ChatMessage {
    fn into(self) -> Message {
        if let Ok(s) = serde_json::to_string(&self) {
            Message::Text(s.into())
        } else {
            Message::Text("{}".into())
        }
    }
}

impl<'a> DBSchema<'a> for ChatMessage {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(DatabaseId::field_definition("conversation_id", false)),
                Arc::new(DatabaseId::field_definition("agent_id", true)),
                Arc::new(ChatParticipant::field_definition("sender", true)),
                Arc::new(String::field_definition("body", false)),
                Arc::new(DateTime::field_definition("ctime", false))])
    }
}

impl<'a> DBEntity<'a, DatabaseId> for ChatMessage {
    type PartialUpdateType = ChatMessage;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::ChatMessage
    }

    fn merge_keys() -> &'static [&'static str] {
        &["id"]
    }

    fn primary_key() -> &'static str {
        "id"
    }

    fn primary_value(&self) -> DatabaseId {
        self.id.clone()
    }
}

impl_default_crud!(ChatMessage, ChatMessage, DatabaseId);

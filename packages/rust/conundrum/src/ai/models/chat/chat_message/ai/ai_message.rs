use std::sync::Arc;

use crate::{
    ai::{
        models::chat::chat_message::user::user_message_input::UserMessageInput,
        rig::ai_traits::ai_chat_history_item::IntoChatHistoryItem,
    },
    ecosystem::db::db_traits::{
        db_entity::{DBEntity, DBSchema},
        db_field::DatabaseField,
    },
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId, static_id::StaticId},
};
use axum::extract::ws::Message;
use fake::Dummy;
use rig::{OneOrMany, message::UserContent};
use serde::{Deserialize, Serialize};

use crate::impl_default_crud;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct AIMessage {
    pub id: DatabaseId,
    pub conversation_id: DatabaseId,
    pub agent_id: DatabaseId,
    pub body: String,
    pub ctime: DateTime,
}

impl Into<Message> for AIMessage {
    fn into(self) -> Message {
        if let Ok(s) = serde_json::to_string(&self) {
            Message::Text(s.into())
        } else {
            Message::Text("{}".into())
        }
    }
}

impl From<UserMessageInput> for AIMessage {
    fn from(value: UserMessageInput) -> Self {
        AIMessage { id: DatabaseId::new(),
                    conversation_id: value.conversation_id.unwrap_or_else(|| DatabaseId::new()),
                    agent_id: value.agent_id.unwrap_or_else(|| {
                                                let id: DatabaseId = StaticId::DefaultAgent.into();
                                                id
                                            }),
                    body: value.body,
                    ctime: DateTime::new_now() }
    }
}

impl Into<rig::prelude::Message> for AIMessage {
    fn into(self) -> rig::prelude::Message {
        rig::prelude::Message::User { content:
                                          OneOrMany::one(UserContent::Text(rig::agent::Text { text: self.body
                                                                                                        .clone(),
                                                                                              additional_params:
                                                                                                  None })) }
    }
}

impl IntoChatHistoryItem for AIMessage {
    fn into_chat_message_history_item(&self) -> String {
        self.body.clone()
    }
}

impl<'a> DBSchema<'a> for AIMessage {
    fn arrow_fields(
        )
        -> crate::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(DatabaseId::field_definition("conversation_id", false)),
                Arc::new(DatabaseId::field_definition("agent_id", true)),
                Arc::new(String::field_definition("body", false)),
                Arc::new(DateTime::field_definition("ctime", false)),])
    }
}

impl<'a> DBEntity<'a, DatabaseId> for AIMessage {
    type PartialUpdateType = AIMessage;

    fn table() -> crate::ecosystem::db::tables::DatabaseTable {
        crate::ecosystem::db::tables::DatabaseTable::AgentMessage
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

impl_default_crud!(AIMessage, AIMessage, DatabaseId);

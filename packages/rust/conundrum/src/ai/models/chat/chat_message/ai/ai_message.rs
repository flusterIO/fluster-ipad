use std::sync::Arc;

use crate::{
    ai::{
        models::{
            agent::agent_primary_task::AgentPrimaryTask, chat::chat_message::user::user_message_input::UserMessageInput,
        },
        rig::ai_traits::{
            ai_chat_history_item::IntoChatHistoryItem, from_with_convo_information::FromWithConvoInformation,
        },
    },
    ecosystem::db::{
        db_traits::{
            db_entity::{DBEntity, DBSchema},
            db_field::DatabaseField,
        },
        tables::DatabaseTable,
    },
    lifted_models::primitives::{
        date_time::DateTime,
        db_id::DatabaseId,
        static_id::{StaticId, StaticIdIter},
    },
};
use axum::extract::ws::Message;
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use rig::{OneOrMany, message::UserContent};
use serde::{Deserialize, Serialize};

use crate::impl_default_crud;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::AgentMessage, source_crate = true)]
pub struct AIMessage {
    pub id: DatabaseId,
    pub convo_id: DatabaseId,
    pub agent_id: DatabaseId,
    pub body: String,
    pub ctime: DateTime,
}

impl FromWithConvoInformation<String> for AIMessage {
    fn from_with_convo_info(data: String, convo_id: DatabaseId, agent_id: Option<DatabaseId>) -> Self {
        AIMessage { id: DatabaseId::new(),
                    convo_id,
                    agent_id: agent_id.unwrap_or_else(|| StaticId::DefaultAgent.into()),
                    body: data,
                    ctime: DateTime::new_now() }
    }
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
                    convo_id: value.convo_id.unwrap_or_else(|| DatabaseId::new()),
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

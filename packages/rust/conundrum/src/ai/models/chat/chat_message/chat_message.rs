use std::sync::Arc;

use axum::extract::ws::Message;
use conundrum_macros::DatabaseEntity;
use crate::{
    ai::{models::chat::chat_sender::chat_sender::ChatParticipant, rig::ai_traits::ai_chat_history_item::IntoChatHistoryItem},
    ecosystem::db::{db_traits::{db_entity::{DBEntity, DBSchema}, db_field::DatabaseField}}, lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use fake::Dummy;
use rig::{OneOrMany, message::{Reasoning, UserContent}};
use serde::{Deserialize, Serialize};

use crate::{
    impl_default_crud,
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct ChatMessage {
    pub id: DatabaseId,
    pub reasoning_content: Option<String>,
    pub convo_id: DatabaseId,
    /// If the sender is the user, this is the agent requested. If the sender is
    /// AI, this is the AI sending the response.
    pub agent_id: Option<DatabaseId>,
    pub sender: ChatParticipant,
    pub body: String,
    pub ctime: DateTime,
}

impl Into<Message> for ChatMessage {
    fn into(self) -> Message {
        if let Ok(s) = serde_json::to_string(&self) {
            Message::Text(s.into())
        } else {
            Message::Text("{}".into())
        }
    }
}

impl Into<rig::prelude::Message> for ChatMessage {
    fn into(self) -> rig::prelude::Message {
        let agent_id_string = self.agent_id.map(|x| x.to_string());
        match self.sender {
            ChatParticipant::User => {
                rig::prelude::Message::User { content: OneOrMany::one(
                    UserContent::Text(
                    rig::agent::Text { text: self.body.clone(), additional_params: None }
                    )
                ) }
            }
            ChatParticipant::SystemPrompt => {
                rig::prelude::Message::System { content: self.body.clone() }
            }
            ChatParticipant::AI => {
                rig::prelude::Message::Assistant { id: agent_id_string.clone(), content: match self.reasoning_content {
                    Some(s) => {
                        OneOrMany::many(vec![
                            rig::message::AssistantContent::Text(rig::agent::Text { text: self.body, additional_params: None}),
                            rig::message::AssistantContent::Reasoning(
                                Reasoning::new_with_signature(s.as_str(), agent_id_string)
                                
                            )
                        ]).expect("Who the fuck puts an error here?")
                    }
                    None => {
                        OneOrMany::one(rig::message::AssistantContent::Text(rig::agent::Text { text: self.body, additional_params: None }))
                    }
                } }
            }
        }
    }
}

impl IntoChatHistoryItem for ChatMessage {
    fn into_chat_message_history_item(&self) -> String {
        self.body.clone()
    }
}

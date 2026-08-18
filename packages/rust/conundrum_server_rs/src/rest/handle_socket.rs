use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use conundrum::ai::models::chat::chat_message::chat_message::ChatMessage;
use conundrum::ai::models::chat::chat_message::user::user_message::UserMessage;
use conundrum::ai::models::{
    agent::agent_primary_task::AgentPrimaryTask, chat::chat_message::user::user_message_input::UserMessageInput,
};
use conundrum::ai::rig::ai_traits::ai_client_container::AIClientContainer;
use conundrum::ai::rig::ai_traits::conundrum_agent::ConundrumAgent;
use conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState;
use futures_util::{SinkExt, StreamExt};
use rig::completion::CompletionModel;

use crate::rig::features::chat::chat_event::ChatEvent;

pub async fn handle_socket<M>(socket: WebSocket, state: Arc<ServerState>)
    where M: CompletionModel {
    if let Some(client) = &state.clone().local_client {
        let (mut tx, mut rx) = socket.split();

        while let Some(result) = rx.next().await {
            let Ok(message) = result else {
                break;
            };

            let Message::Text(prompt) = message else {
                continue;
            };

            let prompt = prompt.to_string();
            let locked_client = client.clone().lock_owned().await;
            let client_result = locked_client.get_default_agent(AgentPrimaryTask::GeneralChat);
            drop(locked_client);
            if let Ok(msg) = serde_json::from_str::<UserMessageInput>(prompt.as_str()) {
                let user_message: UserMessage = UserMessage::from(msg);
                // RESUME: Pick back up here. We won't have to make this generic because the
                // model will always receive a message from the user, so we
                // won't even have to worry about the AIMessage or the
                // SystemPrompt here.
                let mut stream = client_result.stream_chat_response(msg, vec![]).await;
                while let Some(item) = stream.next().await {
                    match item {
                        Ok(data) => {
                            if let Some(event) = ChatEvent::try_from(data).ok() {
                                match serde_json::to_string(&event) {
                                    Ok(s) => {
                                        if let Err(err) = tx.send(Message::text(s)).await {
                                            log::error!("ChatEvent Error: {}", err);
                                        }
                                    }
                                    Err(err) => {
                                        log::error!("ChatEvent Error: {}", err);
                                    }
                                }
                            } else {
                                log::error!("Failed to construct ChatEvent");
                            }
                        }
                        Err(err) => {
                            log::error!("Streaming error: {}", err);
                        }
                    }
                }
            } else {
                log::error!("Streaming error: Failed to deserialize ChatMessage struct.");
            }
        }
    }
}

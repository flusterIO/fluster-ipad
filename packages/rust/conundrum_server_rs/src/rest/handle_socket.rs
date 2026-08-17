use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState;
use futures_util::{SinkExt, StreamExt};
use rig::completion::CompletionModel;
use rig::streaming::StreamingPrompt;

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
            let c = locked_client
            let mut stream = locked_client.0.stream_chat_response(prompt).await;
            drop(locked_client);

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
        }
    }
}

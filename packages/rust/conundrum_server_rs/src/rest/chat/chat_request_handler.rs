use axum::{Json, extract::State};
use conundrum_db::vector::models::ai::chat::chat_message::chat_message::ChatMessage;
use futures_util::stream::{Stream, StreamExt};
use rig::streaming::StreamingChat;
use std::sync::Arc;

use crate::rig::features::chat::chat_event::ChatEvent;

use crate::server_state::ServerState;

pub async fn chat_request_handler(State(state): State<Arc<ServerState>>,
                                  Json(payload): Json<ChatMessage>)
                                  -> impl Stream<Item = ChatEvent> {
    if let Some(local_agent) = state.clone().local_agent.clone() {
        async_stream::stream! {
        let mut stream = local_agent.0.stream_chat::<Vec<String>, String>(&payload.body, vec![]).await;
        while let Some(item) = stream.next().await {
            match item {
                Ok(data) => {
                    if let Ok(item) = ChatEvent::try_from(data) {
                        yield item;
                    }
                },
                Err(err) => {
                    log::error!("Rig error: {}", err);
                    break;
                },
            }
        }
        }
    } else {
        todo!()
    }
}

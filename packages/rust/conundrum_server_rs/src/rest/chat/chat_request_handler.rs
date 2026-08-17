use axum::{Json, extract::State};
use conundrum::ai::rig::ai_traits::conundrum_agent::ConundrumAgent;
use conundrum_db::vector::models::{
    ai::chat::chat_message::chat_message::ChatMessage, ecosystem_data::server_state::server_state::ServerState,
};
use futures_util::stream::{Stream, StreamExt};
use rig::streaming::StreamingChat;
use std::sync::Arc;

use crate::rig::features::chat::chat_event::ChatEvent;

/// # TODO
///
/// - [ ] Read chat history from DB and insert that into the history.
/// - [ ] Save thinking along side but separately from the main history for
///   inserting into history
/// with the proper fields.
pub async fn chat_request_handler(State(state): State<Arc<ServerState>>,
                                  Json(payload): Json<ChatMessage>)
                                  -> impl Stream<Item = ChatEvent> {
    if let Some(local_agent) = state.clone().local_agent.clone() {
        async_stream::stream! {
        let locked_agent = local_agent.clone().lock_owned().await;
        let mut stream = locked_agent.stream_chat_response(payload, vec![]).await;
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

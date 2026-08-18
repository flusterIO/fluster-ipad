use axum::{Json, extract::State};
use conundrum::ai::{
    ai_constants::BASE_TEMPERATURE_CHAT,
    models::{agent::agent_description::AgentDescription, chat::chat_message::chat_message::ChatMessage},
    rig::ai_traits::{ai_client_container::AIClientContainer, conundrum_agent::ConundrumAgent},
};
use conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState;
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
    if let Some(local_client) = state.clone().local_client.clone() {
        async_stream::stream! {
        let locked_client = local_client.clone().lock_owned().await;
        // TODO: Get the agent description from the DB here.
        let agent = locked_client
            .get_agent(AgentDescription::default_local_chat(), BASE_TEMPERATURE_CHAT as f64);
        drop(locked_client);
        let mut stream = agent.stream_chat_response(payload, vec![]).await;
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

use std::fmt::Debug;
use std::sync::{Arc, LazyLock};

use axum::extract::ws::{Message, WebSocket};
use conundrum::ai::models::chat::chat_conversation::chat_conversation::ChatConversation;
use conundrum::ai::models::chat::chat_message::ai::reasoning_block::ReasoningBlock;
use conundrum::ai::models::chat::chat_message::user::user_message::UserMessage;
use conundrum::ai::models::tool::tool_execution::ToolExecution;
use conundrum::ai::models::{
    agent::agent_primary_task::AgentPrimaryTask, chat::chat_message::user::user_message_input::UserMessageInput,
};
use conundrum::ai::rig::ai_traits::ai_client_container::AIClientContainer;
use conundrum::ai::rig::ai_traits::conundrum_agent::ConundrumAgent;
use conundrum::ai::rig::ai_traits::from_with_convo_information::{FromWithConvoInformation, TryFromWithConvoContext};
use conundrum::ai::rig::features::chat::chat_event::ChatEvent;
use conundrum::ai::rig::features::chat::convo_context::ConversationContext;
use conundrum::ecosystem::db::db_traits::entity_crud::EntityCRUD;
use conundrum::lifted_models::primitives::db_id::DatabaseId;
use conundrum::lifted_models::primitives::static_id::StaticId;
use conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState;
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use rig::agent::MultiTurnStreamItem;
use rig::completion::GetTokenUsage;
use rig::streaming::StreamedAssistantContent;
use tokio::sync::Mutex;

static CHAT_CONTEXT: LazyLock<Arc<Mutex<ConversationContext>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(ConversationContext { convo: DatabaseId::new(),
                                              agent: StaticId::DefaultAgent.into(),
                                              accumulator: Arc::new(Mutex::new(DashMap::new())),
                                              reasoning_accumulator: Arc::new(Mutex::new(DashMap::new())) }))
});

pub async fn handle_socket(socket: WebSocket, state: Arc<ServerState>) {
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
            let client_result = locked_client.get_default_agent(AgentPrimaryTask::Agent);
            drop(locked_client);
            if let Ok(msg) = serde_json::from_str::<UserMessageInput>(prompt.as_str()) {
                let mut ctx = CHAT_CONTEXT.clone().lock_owned().await;
                ctx.agent = msg.clone().agent_id.unwrap_or_else(|| StaticId::DefaultAgent.into());
                if let Some(cid) = &msg.convo_id {
                    ctx.convo = cid.clone();
                    drop(ctx);
                    let conversation = ChatConversation::new(cid.clone(), None);
                    let _ = conversation.make_require_update(&Arc::clone(&state.db))
                                        .await
                                        .inspect_err(|e| {
                                            log::error!("Conversation Error: {:#?}", e);
                                        });
                } else {
                    drop(ctx);
                }
                let user_message: UserMessage = UserMessage::from(msg);
                let db = Arc::clone(&state.db);
                let _ = UserMessage::save_many(vec![user_message.clone()], &Arc::clone(&db)).await.inspect_err(|e| {
                    log::error!("Conundrum failed attempting to save the submitted user message: {:#?}", e);
                });
                let mut stream = client_result.stream_chat_response(user_message, vec![]).await;
                while let Some(item) = stream.next().await {
                    match item {
                        Ok(data) => {
                            match ChatEvent::try_from_with_convo_info(data, &Arc::clone(&CHAT_CONTEXT)).await {
                                Ok(event) => {
                                    let ctx = Arc::clone(&CHAT_CONTEXT);
                                    let db_clone = Arc::clone(&state.db);
                                    let _ = event.side_effect(ctx, db_clone).await.inspect_err(|e| {
                                                                                      log::error!("Error: {:#?}", e);
                                                                                  });
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
                                }
                                Err(err) => {
                                    log::error!("Failed to construct ChatEvent: {:#?}", err);
                                }
                            };
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

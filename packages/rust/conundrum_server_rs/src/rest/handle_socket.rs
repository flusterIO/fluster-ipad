use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use conundrum::ai::models::chat::chat_conversation::chat_conversation::ChatConversation;
use conundrum::ai::models::chat::chat_message::ai::ai_message::AIMessage;
use conundrum::ai::models::chat::chat_message::ai::reasoning_block::ReasoningBlock;
use conundrum::ai::models::chat::chat_message::user::user_message::UserMessage;
use conundrum::ai::models::tool::tool_execution::ToolExecution;
use conundrum::ai::models::{
    agent::agent_primary_task::AgentPrimaryTask, chat::chat_message::user::user_message_input::UserMessageInput,
};
use conundrum::ai::rig::ai_traits::ai_client_container::AIClientContainer;
use conundrum::ai::rig::ai_traits::conundrum_agent::ConundrumAgent;
use conundrum::ai::rig::ai_traits::from_with_convo_information::{
    FromWithConvoInformation, TryFromWithConvoInformation,
};
use conundrum::ai::rig::features::chat::chat_event::ChatEvent;
use conundrum::ecosystem::db::db_traits::entity_crud::EntityCRUD;
use conundrum::lifted_models::primitives::db_id::DatabaseId;
use conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState;
use futures_util::{SinkExt, StreamExt};
use rig::agent::{MultiTurnStreamItem};
use rig::completion::{GetTokenUsage};
use rig::streaming::{StreamedAssistantContent};
use tokio::sync::{OnceCell};

/// Records the <ConversationId, ActivelyStreamingMessage> in a map until it can
/// be saved.
static ACCUMULATOR: OnceCell<HashMap<DatabaseId, String>> = OnceCell::const_new();

/// ## TODO
/// - [ ] Save tool executions
/// - [x] Save completed messages from AI
/// - [x] Save reasoning content seperately from other AI output.
/// - [ ] Save incoming user messages (in the other function)
async fn handle_side_effects<R>(data: MultiTurnStreamItem<R>,
                                convo_id: DatabaseId,
                                agent_id: Option<DatabaseId>,
                                state: &Arc<ServerState>)
    where R: Clone + Unpin + GetTokenUsage {
    match data {
        MultiTurnStreamItem::ToolExecutionCommitted { tool_call,
                                                      internal_call_id, } => {
            match ToolExecution::try_from_with_convo_info(tool_call, convo_id, agent_id) {
                Ok(r) => {
                    let db = Arc::clone(&state.db);
                    if let Err(err) = ToolExecution::save_many(vec![r], &db).await {
                        log::error!("Failed attempting to save a tool execution. This context will be lost in future conversations with this model: {:#?}",
                                    err);
                    }
                }
                Err(err) => {
                    log::error!("Failed attempting to serialize a tool execution. This context will be lost in future conversations with this model: {:#?}",
                                err);
                }
            }
        }
        MultiTurnStreamItem::StreamAssistantItem(content) => match content {
            StreamedAssistantContent::Reasoning(x) => {
                let reasoning_block = ReasoningBlock::from_with_convo_info(x, convo_id, agent_id);
                let db = Arc::clone(&state.db);
                let _ = ReasoningBlock::save_many(vec![reasoning_block], &db).await
                    .inspect_err(|e| {
                        log::error!("Failed trying to save agent's reasoning block. This context will be lost in future conversations: {:#?}", e);
                    });
            }
            StreamedAssistantContent::Text(text) => {
                let mut accumulator = ACCUMULATOR.get_or_init(|| async { HashMap::new() }).await.clone();
                let next_content = match accumulator.get_mut(&convo_id) {
                    Some(q) => {
                        println!("Q: {}", q.clone());
                        return *q += text.text.as_str();
                    }
                    None => text.text.clone(),
                };
                accumulator.insert(convo_id.clone(), next_content);
            }
            _ => {}
        },
        MultiTurnStreamItem::FinalResponse(pr) => {
            let msg = AIMessage::from_with_convo_info(pr.output.clone(), convo_id, agent_id);
            let db = Arc::clone(&state.db);
            let _ = AIMessage::save_many(vec![msg], &db).await.inspect_err(|e| {
                log::error!("Failed to save an Agent generated message. This context will be lost in future conversations.");
            });
        }
        _ => {}
    }
}

pub async fn handle_socket(socket: WebSocket, state: Arc<ServerState>) {
    let mut conversation_id = DatabaseId::new();
    let mut agent_id: Option<DatabaseId>;
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
                if let Some(cid) = &msg.convo_id {
                    conversation_id = cid.clone();
                    let conversation = ChatConversation::new(cid.clone(), None);
                    let _ = conversation.make_require_update(&Arc::clone(&state.db))
                                        .await
                                        .inspect_err(|e| {
                                            log::error!("Conversation Error: {:#?}", e);
                                        });
                }
                agent_id = msg.agent_id.clone();
                let user_message: UserMessage = UserMessage::from(msg);
                let db = Arc::clone(&state.db);
                let _ = UserMessage::save_many(vec![user_message.clone()], &Arc::clone(&db)).await.inspect_err(|e| {
                    log::error!("Conundrum failed attempting to save the submitted user message: {:#?}", e);
                });
                // let convo_id = payload.convo_id.as_ref().cloned().unwrap_or_else(|| {
                //     DatabaseId::new()
                // });
                // if payload.convo_id.as_ref().cloned().is_some_and(|x| x == convo_id.clone())
                // || payload.convo_id.is_none() {     payload.convo_id =
                // Some(convo_id.clone()); }
                let mut stream = client_result.stream_chat_response(user_message, vec![]).await;
                while let Some(item) = stream.next().await {
                    match item {
                        Ok(data) => {
                            handle_side_effects(data.clone(),
                                                conversation_id.clone(),
                                                agent_id.clone(),
                                                &Arc::clone(&state)).await;
                            if let Ok(event) =
                                ChatEvent::try_from_with_convo_info(data, conversation_id.clone(), agent_id.clone())
                            {
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

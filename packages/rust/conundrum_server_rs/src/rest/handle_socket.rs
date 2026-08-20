use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
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
use conundrum::ecosystem::db::db_traits::entity_crud::EntityCRUD;
use conundrum::lifted_models::primitives::db_id::DatabaseId;
use conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState;
use futures_util::{SinkExt, StreamExt};
use rig::agent::MultiTurnStreamItem;
use rig::completion::{CompletionModel, GetTokenUsage};
use rig::streaming::{StreamedAssistantContent, StreamingCompletionResponse};

use crate::rig::features::chat::chat_event::ChatEvent;

/// ## TODO
/// - [ ] Save tool executions
/// - [ ] Save completed messages from AI
/// - [?] Save reasoning content seperately from other AI output.
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
                    if let Err(err) = ToolExecution::save_one(r, &db).await {
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
                let _ = ReasoningBlock::save_one(reasoning_block, &db).await
                    .inspect_err(|e| {
                        log::error!("Failed trying to save agent's reasoning block. This context will be lost in future conversations: {:#?}", e);
                    });
            }
            StreamedAssistantContent::Text(text) => {
                println!("The text that needs to be serialized: {}", text.text)
                // let ai_message = AIMessage::from(text.)
            }
            _ => {}
        },
        _ => {}
    }
}

pub async fn handle_socket(socket: WebSocket, state: Arc<ServerState>) {
    let mut conversation_id = DatabaseId::new();
    let mut agent_id = None;
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
                if let Some(cid) = &msg.conversation_id {
                    conversation_id = cid.clone();
                }
                agent_id = msg.agent_id.clone();
                let user_message: UserMessage = UserMessage::from(msg);
                let mut stream = client_result.stream_chat_response(user_message, vec![]).await;
                while let Some(item) = stream.next().await {
                    match item {
                        Ok(data) => {
                            // Turning this on triggers that side-effect issue. Handle this at the
                            // library tomorrow.
                            // handle_side_effects(data.clone(),
                            //                     conversation_id.clone(),
                            //                     agent_id.clone(),
                            //                     &Arc::clone(&state)).await;
                            if let Ok(event) = ChatEvent::try_from(data) {
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

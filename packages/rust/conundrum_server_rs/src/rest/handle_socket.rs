use std::sync::{Arc, LazyLock};

use axum::extract::ws::{Message, WebSocket};
use conundrum::ai::models::{
    agent::{agent_description::AgentDescription, agent_primary_task::AgentPrimaryTask},
    chat::{
        chat_conversation::chat_conversation::ChatConversation,
        chat_message::chat_message::ChatMessage,
        chat_message::user::{user_message::UserMessage, user_message_input::UserMessageInput},
        chat_sender::chat_sender::ChatParticipant,
    },
};
use conundrum::ai::rig::{
    ai_traits::{
        ai_client_container::AIClientContainer,
        conundrum_agent::ConundrumAgent,
        from_with_convo_information::{FromWithConvoInformation, TryFromWithConvoContext},
    },
    features::chat::{chat_event::ChatEvent, convo_context::ConversationContext},
};
use conundrum::ecosystem::{
    db::db_traits::entity_crud::EntityCRUD,
    error_handling::{ai_error::AIError, db_error::DatabaseError},
};
use conundrum::lifted_models::primitives::{db_id::DatabaseId, static_id::StaticId};
use conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState;
use dashmap::DashMap;
use futures_util::{SinkExt, Stream, StreamExt};
use rig::agent::MultiTurnStreamItem;
use tokio::sync::Mutex;

static CHAT_CONTEXT: LazyLock<Arc<Mutex<ConversationContext>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(ConversationContext { convo: DatabaseId::new(),
                                              agent: StaticId::DefaultAgent.into(),
                                              accumulator: Arc::new(Mutex::new(DashMap::new())),
                                              reasoning_accumulator: Arc::new(Mutex::new(DashMap::new())) }))
});

pub async fn handle_socket(socket: WebSocket, state: Arc<ServerState>) {
    let (mut tx, mut rx) = socket.split();
    while let Some(result) = rx.next().await {
        let Ok(message) = result else {
            break;
        };
        let Message::Text(prompt) = message else {
            continue;
        };
        let Ok(msg) = serde_json::from_str::<UserMessageInput>(&prompt) else {
            log::error!("Streaming error: Failed to deserialize user message.");
            continue;
        };

        let model = msg.model.clone();
        {
            let mut ctx = CHAT_CONTEXT.clone().lock_owned().await;
            ctx.agent = msg.agent_id.clone().unwrap_or_else(|| StaticId::DefaultAgent.into());
            if let Some(cid) = &msg.convo_id {
                ctx.convo = cid.clone();
            }
        }
        if let Some(cid) = &msg.convo_id {
            let mut conversation = ChatConversation::new(cid.clone(), None);
            conversation.selected_model = model.clone();
            let _ =
                conversation.make_require_update(state.db.clone()).await.inspect_err(|e| {
                                                                            log::error!("Conversation Error: {:#?}", e);
                                                                        });
        }

        let user_message = UserMessage::from(msg);
        let _ = UserMessage::save_many(vec![user_message.clone()], state.db.clone()).await.inspect_err(|e| {
            log::error!("Conundrum failed attempting to save the submitted user message: {:#?}", e);
        });

        match model.as_ref().map(|m| m.provider.as_str()).unwrap_or("ollama") {
            "ollama" => {
                let Some(client) = &state.local_client else {
                    log::error!("Selected Ollama provider is unavailable.");
                    continue;
                };
                let client = client.clone().lock_owned().await;
                let agent = if let Some(selection) = &model {
                    let mut desc: AgentDescription = AgentPrimaryTask::Agent.into();
                    desc.model = selection.model_id.clone();
                    client.get_agent(desc, AgentPrimaryTask::Agent.to_base_temperature())
                } else {
                    client.get_default_agent(AgentPrimaryTask::Agent)
                };
                drop(client);
                let stream = agent.stream_chat_response(user_message, vec![]).await;
                send_chat_stream(stream, &mut tx, state.clone()).await;
            }
            "openai" => {
                let Some(client) = &state.remote_client else {
                    log::error!("Selected OpenAI provider is unavailable.");
                    continue;
                };
                let Some(selection) = &model else {
                    log::error!("OpenAI model selection is missing.");
                    continue;
                };
                let client = client.clone().lock_owned().await;
                let mut desc = AgentDescription::default_local_chat();
                desc.model = selection.model_id.clone();
                desc.is_local = false;
                let agent = client.get_agent(desc, AgentPrimaryTask::Agent.to_base_temperature());
                drop(client);
                let stream = agent.stream_chat_response(to_chat_message(user_message), vec![]).await;
                send_chat_stream(stream, &mut tx, state.clone()).await;
            }
            provider => log::error!("Unsupported AI provider selected: {provider}"),
        }
    }
}

fn to_chat_message(message: UserMessage) -> ChatMessage {
    ChatMessage { id: message.id,
                  reasoning_content: None,
                  convo_id: message.convo_id,
                  agent_id: Some(message.agent_id),
                  sender: ChatParticipant::User,
                  body: message.body,
                  ctime: message.ctime }
}

async fn send_chat_stream<R, S, E>(mut stream: S,
                                   tx: &mut futures_util::stream::SplitSink<WebSocket, Message>,
                                   state: Arc<ServerState>)
    where R: Clone + Unpin,
          S: Stream<Item = Result<MultiTurnStreamItem<R>, E>> + Unpin,
          E: std::fmt::Display {
    while let Some(item) = stream.next().await {
        match item {
            Ok(data) => match ChatEvent::try_from_with_convo_info(data, &Arc::clone(&CHAT_CONTEXT)).await {
                Ok(event) => {
                    let _ = event.side_effect(Arc::clone(&CHAT_CONTEXT), state.db.clone()).await.inspect_err(|e| {
                        if !matches!(e, DatabaseError::AIError(AIError::SkippingIrrelevantAIOutput)) {
                            log::error!("Error: {:#?}", e);
                        }
                    });
                    match serde_json::to_string(&event) {
                        Ok(s) => {
                            if let Err(err) = tx.send(Message::text(s)).await {
                                log::error!("ChatEvent Error: {err}");
                            }
                        }
                        Err(err) => log::error!("ChatEvent Error: {err}"),
                    }
                }
                Err(err) => {
                    if !matches!(err, DatabaseError::AIError(AIError::SkippingIrrelevantAIOutput)) {
                        log::error!("Failed to construct ChatEvent: {:#?}", err);
                    }
                }
            },
            Err(err) => log::error!("Streaming error: {err}"),
        }
    }
}

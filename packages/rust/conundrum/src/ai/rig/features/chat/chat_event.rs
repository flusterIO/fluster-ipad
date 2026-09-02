use std::sync::Arc;

use fake::Dummy;
use rig::{
    message::{MimeType, ReasoningContent},
    streaming::StreamedAssistantContent,
};
use serde::Serialize;

use crate::{
    ai::{
        models::{
            chat::chat_message::{
                ai::{ai_message::AIMessage, reasoning_block::ReasoningBlock},
                user::user_message::UserMessage,
            },
            tool::tool_execution::ToolExecution,
        },
        rig::{
            ai_traits::from_with_convo_information::TryFromWithConvoContext,
            ai_types::ai_types::LocalMultiTurnStreamItem,
            features::chat::{convo_context::ArcMutexConversationContext, log_usage::log_usage},
        },
    },
    ecosystem::{
        db::{db_client::db_client::DBClient, db_traits::entity_crud::EntityCRUD},
        error_handling::{
            ai_error::AIError,
            db_error::{DatabaseError, DatabaseResult},
        },
    },
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone, specta::Type, Dummy)]
#[specta(export = true)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "content")]
pub enum ChatEvent {
    /// Returned directly after the user sends their message, only attaching an
    /// id and the ctime.
    UserMessageBounceBack {
        user_message: UserMessage,
    },
    TextDelta {
        text: String,
        is_reasoning: bool,
    },
    Redacted {
        text: String,
    },
    /// A complete block of reasoning logic.
    ReasoningBlock {
        text: String,
    },
    /// A summary of the reasoning content.
    ReasoningSummary {
        text: String,
    },
    Encrypted {
        text: String,
    },
    Done {
        input_tokens: u32,
        output_tokens: u32,
        total_tokens: u32,
    },
    ToolCall(ToolExecution),
    ToolResultText {
        content: String,
    },
    ToolResultImage {
        image_type: Option<String>,
    },
    UserContent {
        text: String,
    },
    Many(Vec<ChatEvent>),
}

impl ChatEvent {
    pub async fn side_effect(&self, context: ArcMutexConversationContext, database: DBClient) -> DatabaseResult<()> {
        match self {
            Self::ToolCall(c) => {
                ToolExecution::save_many(vec![c.clone()], database.clone()).await?;
                Ok(())
            }
            Self::ReasoningBlock { text, } => {
                let mut ctx = context.clone().lock_owned().await;
                let reasoning_block = ReasoningBlock { id: DatabaseId::new(),
                                                       convo_id: ctx.convo.clone(),
                                                       agent_id: ctx.agent.clone(),
                                                       content: text.clone(),
                                                       ctime: DateTime::new_now() };
                ReasoningBlock::save_many(vec![reasoning_block], database.clone()).await?;
                ctx.clear_current_reasoning_accumulator();
                drop(ctx);
                Ok(())
            }
            Self::TextDelta { text,
                              is_reasoning, } => {
                let ctx = context.clone().lock_owned().await;
                let accumulator = match is_reasoning {
                    true => ctx.reasoning_accumulator.clone().lock_owned().await,
                    false => ctx.accumulator.clone().lock_owned().await,
                };
                let next_content: String = match accumulator.get_mut(&ctx.convo) {
                    Some(q) => {
                        format!("{}{}", q.clone(), &text)
                    }
                    None => text.clone(),
                };
                accumulator.insert(ctx.convo.clone(), next_content);
                drop(ctx);
                drop(accumulator);
                Ok(())
            }
            Self::Done { .. } => {
                let ctx = context.clone().lock_owned().await;
                let accumulator = ctx.accumulator.clone().lock_owned().await;
                let reasoning_accumulator = ctx.reasoning_accumulator.clone().lock_owned().await;
                if let Some(reasoning_content) = reasoning_accumulator.get(&ctx.convo) {
                    let reasoning_block = ReasoningBlock { id: DatabaseId::new(),
                                                           convo_id: ctx.convo.clone(),
                                                           agent_id: ctx.agent.clone(),
                                                           content: reasoning_content.clone(),
                                                           ctime: DateTime::new_now() };
                    ReasoningBlock::save_many(vec![reasoning_block], database.clone()).await?;
                } else {
                    log::warn!("Attempted to save a reasoning block but could not find any content. If you have reasoning turned off ignore this warning, otherwise this may indicate an issue.");
                }
                if let Some(content) = accumulator.get(&ctx.convo) {
                    let agent_message = AIMessage { id: DatabaseId::new(),
                                                    body: content.clone(),
                                                    convo_id: ctx.convo.clone(),
                                                    agent_id: ctx.agent.clone(),
                                                    ctime: DateTime::new_now() };
                    AIMessage::save_many(vec![agent_message], database.clone()).await?;
                } else {
                    log::warn!("Attempted to save an agent message but could not find any content.");
                }
                accumulator.remove(&ctx.convo);
                reasoning_accumulator.remove(&ctx.convo);
                drop(ctx);
                drop(reasoning_accumulator);
                drop(accumulator);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl<R> TryFromWithConvoContext<StreamedAssistantContent<R>> for ChatEvent where R: Clone + Unpin {
    async fn try_from_with_convo_info(value: StreamedAssistantContent<R>,
                                      ctx: &ArcMutexConversationContext)
                                      -> crate::ecosystem::error_handling::db_error::DatabaseResult<Self>
        where Self: Sized {
        match value {
            StreamedAssistantContent::ReasoningDelta { reasoning,
                                                       .. } => Ok(ChatEvent::TextDelta { text: reasoning,
                                                                                         is_reasoning: true }),

            StreamedAssistantContent::Text(text) => Ok(ChatEvent::TextDelta { text: text.text,
                                                                              is_reasoning: false }),

            StreamedAssistantContent::Reasoning(reasoning) => {
                let events =
                    reasoning.content
                             .into_iter()
                             .filter_map(|content| match content {
                                 ReasoningContent::Text { text,
                                                          .. } => Some(ChatEvent::TextDelta { text,
                                                                                              is_reasoning: true }),

                                 ReasoningContent::Redacted { data, } => Some(ChatEvent::Redacted { text: data }),

                                 ReasoningContent::Summary(text) => Some(ChatEvent::ReasoningSummary { text }),

                                 ReasoningContent::Encrypted(text) => Some(ChatEvent::Encrypted { text }),

                                 _ => {
                                     log::debug!("Encountered unsupported reasoning content");
                                     None
                                 }
                             })
                             .collect::<Vec<_>>();

                if events.is_empty() {
                    log::debug!("Skipping empty reasoning content");
                    Err(DatabaseError::AIError(AIError::SkippingIrrelevantAIOutput))
                } else if events.len() == 1 {
                    Ok(events.into_iter().next().unwrap())
                } else {
                    Ok(ChatEvent::Many(events))
                }
            }

            StreamedAssistantContent::ToolCall { tool_call,
                                                 .. } => {
                let r = ToolExecution::try_from_with_convo_info(tool_call, &Arc::clone(&ctx)).await?;
                Ok(ChatEvent::ToolCall(r))
            }

            StreamedAssistantContent::Final(_) => Err(DatabaseError::AIError(AIError::SkippingIrrelevantAIOutput)),

            StreamedAssistantContent::Unknown(val) => {
                log::debug!("Found unknown LLM output: {:#?}", val);
                Err(DatabaseError::AIError(AIError::SkippingIrrelevantAIOutput))
            }

            StreamedAssistantContent::ToolCallDelta { .. } => {
                log::debug!("Skipping tool call delta");
                Err(DatabaseError::AIError(AIError::SkippingIrrelevantAIOutput))
            }
        }
    }
}

impl TryFromWithConvoContext<LocalMultiTurnStreamItem> for ChatEvent {
    async fn try_from_with_convo_info(value: LocalMultiTurnStreamItem,
                                      ctx: &ArcMutexConversationContext)
                                      -> crate::ecosystem::error_handling::db_error::DatabaseResult<Self>
        where Self: Sized {
        match value {
            rig::agent::MultiTurnStreamItem::StreamAssistantItem(x) => {
                if let Ok(res) = ChatEvent::try_from_with_convo_info(x, &Arc::clone(ctx)).await {
                    Ok(res)
                } else {
                    log::warn!("Something went wrong while gathering a ChatEvent. Cannot stream this event to the front-end.");
                    Err(DatabaseError::AIError(AIError::SkippingIrrelevantAIOutput))
                }
            }
            rig::agent::MultiTurnStreamItem::FinalResponse(x) => {
                let usage = x.usage();
                Ok(ChatEvent::Done { input_tokens: usage.input_tokens as u32,
                                     output_tokens: usage.output_tokens as u32,
                                     total_tokens: usage.total_tokens as u32 })
            }
            rig::agent::MultiTurnStreamItem::StreamUserItem(x) => match x {
                rig::streaming::StreamedUserContent::ToolResult { tool_result,
                                                                  .. } => {
                    let mut results = Vec::new();
                    for k in tool_result.content {
                        match k {
                            rig::message::ToolResultContent::Text(t) => {
                                results.push(ChatEvent::ToolResultText { content: t.text });
                            }
                            rig::message::ToolResultContent::Json { value, } => {
                                if let Ok(content) = serde_json::to_string(&value) {
                                    results.push(ChatEvent::ToolResultText { content });
                                } else {
                                    log::warn!("Failed to deserialize ToolResultContent. Cannot pass message to the front-end.");
                                }
                            }
                            rig::message::ToolResultContent::Image(x) => {
                                results.push(ChatEvent::ToolResultImage { image_type:
                                                                       x.media_type
                                                                        .map(|x| x.to_mime_type().to_string()) });
                            }
                        }
                    }
                    Ok(ChatEvent::Many(results))
                }
            },
            rig::agent::MultiTurnStreamItem::ToolExecutionCommitted { tool_call,
                                                                      .. } => {
                let tool_execution =
                    ToolExecution::try_from_with_convo_info(tool_call.clone(), &Arc::clone(ctx)).await?;
                Ok(Self::ToolCall(tool_execution))
            }
            rig::agent::MultiTurnStreamItem::ModelTurnRetried { turn, } => {
                log::info!("Model retried {} times.", turn);
                Err(DatabaseError::AIError(AIError::SkippingIrrelevantAIOutput))
            }
            rig::agent::MultiTurnStreamItem::CompletionCall(c) => {
                log_usage(c.usage);
                Err(DatabaseError::AIError(AIError::SkippingIrrelevantAIOutput))
            }
            _ => {
                log::debug!("Skipping model events that Conundrum doesn't need.");
                Err(DatabaseError::AIError(AIError::SkippingIrrelevantAIOutput))
            }
        }
    }
}

use fake::Dummy;
use rig::{
    message::{MimeType, ReasoningContent},
    streaming::StreamedAssistantContent,
};
use serde::Serialize;

use crate::{
    ai::{
        models::{chat::chat_message::user::user_message::UserMessage, tool::tool_execution::ToolExecution},
        rig::{
            ai_traits::from_with_convo_information::TryFromWithConvoInformation,
            ai_types::ai_types::LocalMultiTurnStreamItem,
        },
    },
    ecosystem::error_handling::{ai_error::AIError, db_error::DatabaseError},
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

impl<R> TryFromWithConvoInformation<StreamedAssistantContent<R>> for ChatEvent where R: Clone + Unpin {
    fn try_from_with_convo_info(value: StreamedAssistantContent<R>,
                                convo_id: crate::lifted_models::primitives::db_id::DatabaseId,
                                agent_id: Option<crate::lifted_models::primitives::db_id::DatabaseId>)
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
                    Err(DatabaseError::AIError(AIError::SkippingIrrelevantAIOutput))
                } else if events.len() == 1 {
                    Ok(events.into_iter().next().unwrap())
                } else {
                    Ok(ChatEvent::Many(events))
                }
            }

            StreamedAssistantContent::ToolCall { tool_call,
                                                 .. } => {
                let r = ToolExecution::try_from_with_convo_info(tool_call, convo_id.clone(), agent_id.clone())?;
                Ok(ChatEvent::ToolCall(r))
            }

            StreamedAssistantContent::Final(_) => Err(DatabaseError::AIError(AIError::SkippingIrrelevantAIOutput)),

            _ => {
                log::debug!("Skipping unknown AI output.");
                Err(DatabaseError::AIError(AIError::SkippingIrrelevantAIOutput))
            }
        }
    }
}

// impl TryFrom<StreamedAssistantContent<ollama::StreamingCompletionResponse>>
// for ChatEvent {     type Error = ServerError;

//     fn try_from(value:
// StreamedAssistantContent<ollama::StreamingCompletionResponse>) ->
// Result<Self, Self::Error> {         match value {
//             StreamedAssistantContent::ReasoningDelta { id,
//                                                        reasoning, } =>
// Ok(ChatEvent::TextDelta { text: reasoning,
// is_reasoning: true }),             StreamedAssistantContent::Text(s) =>
// Ok(ChatEvent::TextDelta { text: s.text,
// is_reasoning: false }),             StreamedAssistantContent::Reasoning(r) =>
// {                 let mut items: Vec<ChatEvent> = Vec::new();
//                 for x in r.content {
//                     match x {
//                         ReasoningContent::Text { text,
//                                                  .. } =>
// items.push(ChatEvent::TextDelta { text,
// is_reasoning: true }),                         ReasoningContent::Redacted {
// data, } => items.push(ChatEvent::Redacted { text: data }),
// ReasoningContent::Summary(s) => items.push(ChatEvent::ReasoningSummary {
// text: s }),                         ReasoningContent::Encrypted(x) =>
// items.push(ChatEvent::Encrypted { text: x }),                         _ => {
//                             log::debug!("Encountered some piece of mystery AI
// output...");                         }
//                     }
//                 }
//                 Ok(Self::Many(items))
//             }
//             StreamedAssistantContent::ToolCall { tool_call,
//                                                  .. } => {
//                 let tool_name = tool_call.function.name;
//                 let tool_input_params =
// serde_json::to_string(&tool_call.function.arguments).ok();
// Ok(ChatEvent::ToolCall { tool_name,
// tool_input_params })             }
//             StreamedAssistantContent::Final(_) => {
//                 // let usage = x.token_usage();
//                 // let input_tokens = usage.input_tokens as u32;
//                 // let output_tokens = usage.output_tokens as u32;
//                 // let total_tokens = usage.total_tokens as u32;
//                 // Ok(ChatEvent::Done { input_tokens,
//                 //                      total_tokens,
//                 //                      output_tokens })
//                 Err(ServerError::SkippingIrrelevantAIOutput)
//             }
//             _ => {
//                 log::debug!("Skipping unknown AI output.");
//                 Err(ServerError::SkippingIrrelevantAIOutput)
//             }
//         }
//     }
// }

impl TryFromWithConvoInformation<LocalMultiTurnStreamItem> for ChatEvent {
    fn try_from_with_convo_info(value: LocalMultiTurnStreamItem,
                                convo_id: crate::lifted_models::primitives::db_id::DatabaseId,
                                agent_id: Option<crate::lifted_models::primitives::db_id::DatabaseId>)
                                -> crate::ecosystem::error_handling::db_error::DatabaseResult<Self>
        where Self: Sized {
        match value {
            rig::agent::MultiTurnStreamItem::StreamAssistantItem(x) => {
                if let Ok(res) = ChatEvent::try_from_with_convo_info(x, convo_id.clone(), agent_id.clone()) {
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
                    ToolExecution::try_from_with_convo_info(tool_call.clone(), convo_id.clone(), agent_id.clone())?;
                Ok(Self::ToolCall(tool_execution))
            }
            _ => {
                log::debug!("Skipping model events that Conundrum doesn't need.");
                Err(DatabaseError::AIError(AIError::SkippingIrrelevantAIOutput))
            }
        }
    }
}

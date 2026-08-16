use fake::Dummy;
use rig::{
    message::{MimeType, ReasoningContent},
    streaming::StreamedAssistantContent,
};
use serde::Serialize;
use typeshare::typeshare;

use crate::{errors::server_error::ServerError, rig::ai_types::ai_types::LocalMultiTurnStreamItem};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone, specta::Type, Dummy)]
#[specta(export = true)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type", content = "content")]
pub enum ChatEvent {
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
    ToolCall {
        tool_name: String,
        tool_input_params: Option<String>,
    },
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

impl<R> TryFrom<StreamedAssistantContent<R>> for ChatEvent where R: Clone + Unpin {
    type Error = ServerError;

    fn try_from(value: StreamedAssistantContent<R>) -> Result<Self, Self::Error> {
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
                    Err(ServerError::SkippingIrrelevantAIOutput)
                } else if events.len() == 1 {
                    Ok(events.into_iter().next().unwrap())
                } else {
                    Ok(ChatEvent::Many(events))
                }
            }

            StreamedAssistantContent::ToolCall { tool_call,
                                                 .. } => {
                Ok(ChatEvent::ToolCall { tool_name: tool_call.function.name,
                                         tool_input_params:
                                             serde_json::to_string(&tool_call.function.arguments).ok() })
            }

            StreamedAssistantContent::Final(_) => Err(ServerError::SkippingIrrelevantAIOutput),

            _ => {
                log::debug!("Skipping unknown AI output.");
                Err(ServerError::SkippingIrrelevantAIOutput)
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

impl TryFrom<LocalMultiTurnStreamItem> for ChatEvent {
    type Error = ServerError;

    fn try_from(value: LocalMultiTurnStreamItem) -> Result<Self, Self::Error> {
        match value {
            rig::agent::MultiTurnStreamItem::StreamAssistantItem(x) => {
                if let Ok(res) = ChatEvent::try_from(x) {
                    Ok(res)
                } else {
                    log::warn!("Something went wrong while gathering a ChatEvent. Cannot stream this event to the front-end.");
                    Err(ServerError::SkippingIrrelevantAIOutput)
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
                let tool_input_params = serde_json::to_string(&tool_call.function.arguments).ok();
                Ok(ChatEvent::ToolCall { tool_name: tool_call.function.name,
                                         tool_input_params })
            }
            _ => {
                log::debug!("Skipping model events that Conundrum doesn't need.");
                Err(ServerError::SkippingIrrelevantAIOutput)
            }
        }
    }
}

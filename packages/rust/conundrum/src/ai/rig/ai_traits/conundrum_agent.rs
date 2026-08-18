use rig::agent::StreamingPromptRequest;
use rig::client::EmbeddingsClient;
use rig::completion::{CompletionModel, GetTokenUsage};
use rig::embeddings::EmbeddingModel;
use rig::prelude::StreamingChat;
use rig::{Agent, message::Message};

use crate::ai::rig::ai_traits::ai_chat_history_item::IntoChatHistoryItem;

pub trait ConundrumAgent
    where Self::CompletionModelType: 'static {
    type EmbeddingsClientType: EmbeddingsClient;
    type CompletionModelType: CompletionModel + Sized;
    type EmbeddingModelType: EmbeddingModel;
    type ChatMessageModel: Into<Message> + IntoChatHistoryItem;

    fn inner_agent(&self) -> Agent<Self::CompletionModelType>;
    fn default_embedding_model_name() -> &'static str;
    fn stream_chat_response(&self,
                            chat_request: Self::ChatMessageModel,
                            chat_history: Vec<Self::ChatMessageModel>)
                            -> StreamingPromptRequest<Self::CompletionModelType> {
        todo!()
    }
}

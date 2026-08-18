use crate::ai::models::chat::chat_message::user::user_message::UserMessage;
use crate::ai::{
    models::chat::chat_message::chat_message::ChatMessage, rig::ai_traits::ai_chat_history_item::IntoChatHistoryItem,
};
use rig::{
    Agent, agent::StreamingPromptRequest, client::AgentClientExt, message::Message, providers::ollama,
    streaming::StreamingChat,
};

use crate::ai::rig::ai_traits::{conundrum_agent::ConundrumAgent, from_agent_description::FromAgentDescription};

use crate::ai::models::agent::agent_description::AgentDescription;

#[derive(Clone)]
pub struct LocalAgent(pub Agent<ollama::CompletionModel>);

impl FromAgentDescription<ollama::Client> for LocalAgent {
    fn from_agent_description(client: ollama::Client,
                              agent_desc: AgentDescription,
                              task_base_temperature: f64)
                              -> Self {
        let agent = client
            .agent(agent_desc.model)
            .temperature(task_base_temperature * (agent_desc.temperature_scalar as f64))
            .preamble(agent_desc.instructions.unwrap_or("You are an assistant for an ecosystem academic note taking tools for STEM students and professionals.".to_string()).as_str())
            .build();
        Self(agent)
    }
}

impl ConundrumAgent for LocalAgent {
    type ChatMessageModel = UserMessage;
    type CompletionModelType = ollama::CompletionModel;
    type EmbeddingModelType = ollama::EmbeddingModel;
    type EmbeddingsClientType = ollama::Client;

    fn default_embedding_model_name() -> &'static str {
        // TODO: Actually do something dynamic here. Check the user's machine and
        // recommend a model based on their hardware.
        "qwen3:8b"
    }

    fn inner_agent(&self) -> Agent<Self::CompletionModelType> {
        self.0.clone()
    }

    fn stream_chat_response(&self,
                            chat_request: Self::ChatMessageModel,
                            chat_history: Vec<Self::ChatMessageModel>)
                            -> StreamingPromptRequest<Self::CompletionModelType> {
        let model = self.inner_agent();
        let history = chat_history.iter().map(|x| x.into_chat_message_history_item()).collect::<Vec<String>>();
        let m: Message = chat_request.into();
        model.stream_chat::<Vec<String>, String>(m, history)
    }
}

use crate::ai::{
    models::{agent::agent_description::AgentDescription, chat::chat_message::chat_message::ChatMessage},
    rig::ai_types::ai_types::{RemoteCompletionClient, RemoteCompletionModel},
};
use rig::{
    Agent,
    client::AgentClientExt,
    providers::openai::{self, embedding::TEXT_EMBEDDING_3_LARGE},
};

use crate::ai::rig::ai_traits::{conundrum_agent::ConundrumAgent, from_agent_description::FromAgentDescription};

#[derive(Clone)]
pub struct RemoteAgent(pub Agent<RemoteCompletionModel>);

impl FromAgentDescription<RemoteCompletionClient> for RemoteAgent {
    fn from_agent_description(client: RemoteCompletionClient,
                              agent_desc: AgentDescription,
                              task_base_temperature: f64)
                              -> Self {
        let agent  = client
            .agent(agent_desc.model)
            .temperature(task_base_temperature * (agent_desc.temperature_scalar as f64))
            .preamble(agent_desc.instructions.unwrap_or("You are an assistant for an ecosystem of academic note taking tools for STEM students and professionals.".to_string()).as_str())
            .build();
        Self(agent)
    }
}

impl ConundrumAgent for RemoteAgent {
    type ChatMessageModel = ChatMessage;
    type CompletionModelType = RemoteCompletionModel;
    type EmbeddingModelType = openai::embedding::EmbeddingModel;
    type EmbeddingsClientType = openai::Client;

    fn default_embedding_model_name() -> &'static str {
        TEXT_EMBEDDING_3_LARGE
    }

    fn inner_agent(&self) -> Agent<Self::CompletionModelType> {
        self.0.clone()
    }
}

use conundrum_db::vector::models::ai::agent::agent_description::AgentDescription;
use rig::{
    Agent,
    client::{AgentClientExt, ProviderClient},
    providers::ollama,
};

use crate::{
    errors::server_error::{ServerError, ServerResult},
    rig::{
        ai_traits::{conundrum_agent::ConundrumAgent, from_agent_description::FromAgentDescription},
        ai_types::ai_types::LocalCompletionModel,
    },
};

#[derive(Clone)]
pub struct LocalAgent(pub Agent<ollama::CompletionModel>);

impl FromAgentDescription for LocalAgent {
    fn from_agent_description(agent_desc: AgentDescription) -> ServerResult<Self> {
        let client = ollama::Client::from_env().map_err(|e| {
                                                   log::error!("Ollama Error: {:#?}", e);
                                                   ServerError::LocalAgentFailToConnect
                                               })?;
        let agent = client
            .agent(agent_desc.model)
            .preamble(agent_desc.instructions.unwrap_or("You are an assistant for an ecosystem academic note taking tools for STEM students and professionals.".to_string()).as_str())
            .build();
        Ok(Self(agent))
    }
}

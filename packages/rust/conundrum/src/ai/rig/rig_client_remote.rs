use crate::{
    ai::{
        agents::remote_agent::RemoteAgent,
        models::agent::agent_description::AgentDescription,
        rig::ai_traits::{
            ai_client_container::{AIClientContainer, AIClientEmbedder},
            from_agent_description::FromAgentDescription,
            into_embedding_description::IntoEmbeddingDescription,
        },
    },
    ecosystem::error_handling::ai_error::{AIError, AIResult},
};
use rig::{
    client::{EmbeddingsClient, ProviderClient},
    providers::openai::{Client as OpenAIClient, TEXT_EMBEDDING_3_LARGE},
};

const DEFAULT_REMOTE_EMBEDDING_MODEL: &str = TEXT_EMBEDDING_3_LARGE;

/// # RigProvider
///
/// A general purpose wrapper around whichever Rig provider makes it into the
/// initial release, because I can't afford internet, much less provider tokens.
pub struct RigClientRemote(pub OpenAIClient);

impl RigClientRemote {
    pub fn initialize() -> AIResult<Self> {
        let client = OpenAIClient::from_env().map_err(|e| {
                                                 log::error!("Failed to initialize model with the error: {:?}", e);
                                                 AIError::FailToInitializeModel("OpenAI language".to_string())
                                             })?;
        Ok(RigClientRemote(client))
    }
}

impl AIClientContainer for RigClientRemote {
    type AgentContainer = RemoteAgent;

    fn get_embedding_model(&self, name: Option<String>, ndims: Option<usize>) -> impl rig::prelude::EmbeddingModel {
        let model_name = name.unwrap_or(DEFAULT_REMOTE_EMBEDDING_MODEL.to_string());
        if let Some(dims) = ndims {
            self.0.embedding_model_with_ndims(model_name, dims)
        } else {
            self.0.embedding_model(model_name)
        }
    }

    fn get_default_agent(&self,
                         task: crate::ai::models::agent::agent_primary_task::AgentPrimaryTask)
                         -> Self::AgentContainer {
        // TODO: Actually implement a match against some models for the various
        // tasks when you're on internet for good.
        let client = self.0.clone();
        RemoteAgent::from_agent_description(client, AgentDescription::default_local_chat(), task.to_base_temperature())
    }

    fn get_agent(&self, desc: AgentDescription, task_base_temperature: f64) -> Self::AgentContainer {
        let client = self.0.clone();
        RemoteAgent::from_agent_description(client, desc, task_base_temperature)
    }

    async fn validate_environment() -> AIResult<()> {
        // TODO: Replace this with just reading an evironment variable when you're
        // online an can figure out what the valid environment variables are.
        let env = OpenAIClient::from_env().map_err(|e| {
                                              log::error!("Error: {:#?}", e);
                                              AIError::InvalidEnvironment("openai".to_string())
                                          })?;
        Ok(())
    }
}

impl<T> AIClientEmbedder<T> for RigClientRemote where T: IntoEmbeddingDescription + Clone + Send + Sync {}

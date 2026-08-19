use crate::{
    ai::{
        agents::local_agent::LocalAgent,
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
    providers::ollama::{self, Client as OllamaClient, OllamaApiKey},
};

const DEFAULT_LOCAL_EMBEDDING_MODEL: &str = "qwen3-embedding:4b";

/// # RigProvider
///
/// A general purpose wrapper around whichever Rig provider makes it into the
/// initial release, because I can't afford internet, much less provider tokens.
pub struct RigClientLocal(pub OllamaClient);

impl RigClientLocal {
    pub fn initialize() -> AIResult<Self> {
        let client = OllamaClient::new(OllamaApiKey::default()).map_err(|e| {
                         log::error!("Failed to initialize model with the error: {:?}", e);
                         AIError::FailToInitializeModel("ollama language".to_string())
                     })?;
        Ok(RigClientLocal(client))
    }
}

impl AIClientContainer for RigClientLocal {
    type AgentContainer = LocalAgent;

    fn get_embedding_model(&self, name: Option<String>, ndims: Option<usize>) -> impl rig::prelude::EmbeddingModel {
        let model_name = name.unwrap_or(DEFAULT_LOCAL_EMBEDDING_MODEL.to_string());
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
        let desc: AgentDescription = task.clone().into();
        LocalAgent::from_agent_description(client, desc, task.to_base_temperature())
    }

    fn get_agent(&self, desc: AgentDescription, task_base_temperature: f64) -> Self::AgentContainer {
        let client = self.0.clone();
        LocalAgent::from_agent_description(client, desc, task_base_temperature)
    }

    async fn validate_environment() -> AIResult<()> {
        /// Figure out the Ollama environment variables when you're online so
        /// you don't have to create an entire client.
        OllamaClient::from_env().map_err(|e| {
                                    log::error!("Invalid Ollama Environment: {:#?}", e);
                                    AIError::InvalidEnvironment("ollama".to_string())
                                })?;
        Ok(())
    }
}

impl<T> AIClientEmbedder<T> for RigClientLocal where T: IntoEmbeddingDescription + Clone + Send + Sync {}

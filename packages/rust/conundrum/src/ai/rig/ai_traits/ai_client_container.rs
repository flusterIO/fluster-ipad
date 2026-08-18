use rayon::prelude::*;
use rig::embeddings::{Embedding, EmbeddingModel};

use crate::{
    ai::{
        models::agent::{agent_description::AgentDescription, agent_primary_task::AgentPrimaryTask},
        rig::ai_traits::{conundrum_agent::ConundrumAgent, into_embedding_description::IntoEmbeddingDescription},
    },
    ecosystem::error_handling::ai_error::{AIError, AIResult},
};

pub trait AIClientContainer {
    type AgentContainer: ConundrumAgent;
    fn get_embedding_model(&self, name: Option<String>, ndims: Option<usize>) -> impl EmbeddingModel;

    fn get_default_agent(&self, task: AgentPrimaryTask) -> Self::AgentContainer;

    fn get_agent(&self, desc: AgentDescription, task_base_temperature: f64) -> Self::AgentContainer;
    /// Throws an error if the environment is invalid, otherwise returns null.
    async fn validate_environment() -> AIResult<()>;
}

pub trait AIClientEmbedder<T>: AIClientContainer
    where T: IntoEmbeddingDescription + Clone + Send + Sync {
    async fn embed_models(&self,
                          embedding_model_name: Option<String>,
                          items: Vec<T>,
                          ndims: Option<usize>)
                          -> AIResult<Vec<Embedding>> {
        let model = self.get_embedding_model(embedding_model_name, ndims);
        let texts = items.par_iter().map(|x| x.into_embedding_description()).collect::<Vec<String>>();
        let r = model.embed_texts(texts).await.map_err(|e| {
                                                   log::error!("Embedding Error: {:#?}", e);
                                                   AIError::EmbeddingFail(T::human_readable_model_name().to_string())
                                               })?;
        Ok(r)
    }
}

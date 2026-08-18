use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;

#[typeshare::typeshare]
#[derive(Debug, Error, uniffi::Error, Serialize, Deserialize, Clone, specta::Type)]
#[serde(tag = "tag", content = "content")]
pub enum AIError {
    #[error("Conundrum Error: {:?}.", .0)]
    ConundrumError(ConundrumErrorVariant),
    #[error("Conundrum failed to initialize the {0} model. We can't complete certain AI related tasks.")]
    FailToInitializeModel(String),
    #[error("Skipping irellevant AI output.")]
    SkippingIrrelevantAIOutput,
    #[error("Conundrum failed to embed a {0} model.")]
    EmbeddingFail(String),
    #[error("Invalid Props: {0}")]
    InvalidProps(String),
    #[error("Conundrum could not find the {0} environment variable.")]
    InvalidEnvironment(String),
    #[error("Conundrum could not connect to the local model.")]
    InvalidLocalProvider,
    #[error("Conundrum could not connect to the remote model.")]
    InvalidRemoteProvider,
}

pub type AIResult<T> where T: Sized
= Result<T, AIError>;

use rig::providers::ollama::{Client as OllamaClient, OllamaApiKey};

use crate::errors::server_error::{ServerError, ServerResult};

/// # RigProvider
/// Deprecated... moved to the Conundrum crate.
pub struct RigClientLocal(pub OllamaClient);

impl RigClientLocal {
    pub fn initialize() -> ServerResult<Self> {
        let client = OllamaClient::new(OllamaApiKey::default()).map_err(|e| {
                         log::error!("Failed to initialize model with the error: {:?}", e);
                         ServerError::ModelInitializationFailure
                     })?;
        Ok(RigClientLocal(client))
    }
}

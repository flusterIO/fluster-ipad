use rig::providers::ollama::Client as OllamaClient;

use crate::errors::server_error::{ServerError, ServerResult};

/// # RigProvider
///
/// A general purpose wrapper around whichever Rig provider makes it into the
/// initial release, because I can't afford internet, much less provider tokens.
pub struct RigClient(pub OllamaClient);

impl RigClient {
    pub fn initialize() -> ServerResult<Self> {
        let client = OllamaClient::new(None).map_err(|e| {
                                                log::error!("Failed to initialize model with the error: {:?}", e);
                                                ServerError::ModelInitializationFailure
                                            })?;
        Ok(RigClient(client))
    }
}

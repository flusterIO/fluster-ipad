use serde::{Deserialize, Serialize};
use thiserror::Error;

#[typeshare::typeshare]
#[derive(Debug, Error, uniffi::Error, Serialize, Deserialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum ConfigError {
    #[error("This error should never make it back to the user, but I'm in a hurry...")]
    OhShit,
    #[error("Conundrum encountered an error serializing some of your configuration.")]
    SerializationError,
    #[error("Conundrum encountered an error reading a file to your system.")]
    FileReadError,
    #[error("Conundrum encountered an error writing a file to your system. To be safe, let's not continue.")]
    FileWriteError,
    #[error("Conundrum can't find the root directory so it doesn't even know where to start!")]
    FailToFindProjectRoot,
}

pub type ConfigResult<T> = Result<T, ConfigError>;

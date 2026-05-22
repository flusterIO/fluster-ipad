use serde::{Deserialize, Serialize};
use thiserror::Error;

#[typeshare::typeshare]
#[derive(Debug, Error, uniffi::Error, Serialize, Deserialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum ConfigError {
    #[error("This error should never make it back to the user, but I'm in a hurry...")]
    OhShit,
}

pub type ConfigResult<T> where T: Sized
= Result<T, ConfigError>;

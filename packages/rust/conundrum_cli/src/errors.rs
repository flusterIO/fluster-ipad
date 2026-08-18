use anyhow::Result;
use conundrum::{
    ecosystem::error_handling::{ai_error::AIError, db_error::DatabaseError},
    lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant,
};
use conundrum_config::errors::config_error::ConfigError;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ConundrumCliError {
    #[error("The `{0}` path is not a directory.")]
    NotADirectory(String),
    #[error("A general file system error occcurred while parsing the `{0}` path.")]
    FsError(String),
    #[error("Conundrum failed to serialize data properly. If this issue persists, this is possibly a bug on our end.")]
    SerializationError,
    #[error("There was an error with your project configuration. There should be further information printed to your console.")]
    ProjectConfigError(Option<String>),
    #[error("The `{0}` path must be a child of the `{1}` path.")]
    FileNotChildOfDir(String, String),
    #[error("Conundrum encountered an error: {0}")]
    ConundrumError(ConundrumErrorVariant),
    #[error("I'm busy as shit.")]
    NotImplemented,
    #[error("Config Error: {0}")]
    ConfigError(ConfigError),
    #[error("AI Error: {0}")]
    AIError(AIError),
    #[error("{0}")]
    DatabaseError(DatabaseError),
}

pub type ConundrumCliResult<T> = Result<T, ConundrumCliError>;

impl From<AIError> for ConundrumCliError {
    fn from(value: AIError) -> Self {
        Self::AIError(value)
    }
}

impl From<DatabaseError> for ConundrumCliError {
    fn from(value: DatabaseError) -> Self {
        Self::DatabaseError(value)
    }
}

impl From<ConfigError> for ConundrumCliError {
    fn from(value: ConfigError) -> Self {
        Self::ConfigError(value)
    }
}

impl From<ConundrumErrorVariant> for ConundrumCliError {
    fn from(value: ConundrumErrorVariant) -> Self {
        Self::ConundrumError(value)
    }
}

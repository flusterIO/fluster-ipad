use anyhow::Result;
use config::ConfigError;
use conundrum::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;
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
}

pub type ConundrumCliResult<T> = Result<T, ConundrumCliError>;

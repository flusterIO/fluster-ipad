use anyhow::Result;
use conundrum::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ConundrumCliError {
    #[error("The `{0}` path is not a directory.")]
    NotADirectory(String),
    #[error("A general file system error occcurred while parsing the `{0}` path.")]
    FsError(String),

    #[error("The `{0}` path must be a child of the `{1}` path.")]
    FileNotChildOfDir(String, String),
    #[error("Conundrum encountered an error: {0}")]
    ConundrumError(ConundrumErrorVariant),
}

pub type ConundrumCliResult<T> = Result<T, anyhow::Error>;

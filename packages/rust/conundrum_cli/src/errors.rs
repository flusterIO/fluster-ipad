use anyhow::Result;
use conundrum::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ConundrumCliError {
    #[error("A general file system error occcurred while parsing the `{self.0}` path.")]
    FsError,
    #[error("The `{self.0}` path is not a directory.")]
    NotADirectory(String),
    #[error("Conundrum encountered an error: {self.0}")]
    ConundrumError(ConundrumErrorVariant),
}

pub type ConundrumCliResult<T> = Result<T, Error>;

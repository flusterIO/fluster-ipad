use conundrum::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[typeshare::typeshare]
#[derive(Debug, Error, uniffi::Error, Serialize, Deserialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum ConundrumFSError {
    #[error("This error should never make it back to the user, but I'm in a hurry...")]
    GeneralFSError,

    #[error("The `{0}` path must be a child of the `{1}` path.")]
    FileNotChildOfDir(String, String),

    #[error("General file system error: {0}")]
    FsError(String),

    #[error("General conundrum error: {0}")]
    ConundrumError(ConundrumErrorVariant),
}

pub type ConundrumFSResult<T> where T: Sized
= Result<T, ConundrumFSError>;

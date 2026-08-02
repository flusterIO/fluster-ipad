use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;

#[typeshare::typeshare]
#[derive(Debug, Error, uniffi::Error, Serialize, Deserialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum ConundrumFSError {
    #[error("This error should never make it back to the user, but I'm in a hurry...")]
    GeneralFSError,

    #[error("Conundrum could not locate the data directory for your operating system. We don't know where to put your data.")]
    InvalidDataDirectory,

    #[error("The `{0}` path must be a child of the `{1}` path.")]
    FileNotChildOfDir(String, String),

    #[error("General file system error: {0}")]
    FsError(String),

    #[error("General conundrum error: {0}")]
    ConundrumError(ConundrumErrorVariant),

    #[error("Conundrum could not serialize a path.")]
    PathSerializationError,

    #[error("Unsupported file extension: {0}.")]
    UnsupportedFileExtension(String),
}

pub type ConundrumFSResult<T> where T: Sized
= Result<T, ConundrumFSError>;

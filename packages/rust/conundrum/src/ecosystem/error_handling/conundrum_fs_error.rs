use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;

#[typeshare::typeshare]
#[derive(Debug, Error, uniffi::Error, Serialize, Deserialize, Clone, specta::Type)]
#[serde(tag = "tag", content = "content")]
pub enum ConundrumFSError {
    #[error("This error should never make it back to the user, but I'm in a hurry...")]
    GeneralFSError,

    #[error("Conundrum could not locate the data directory for your operating system. We don't know where to put your data.")]
    InvalidDataDirectory,

    #[error("The `{child}` path must be a child of the `{parent}` path.")]
    FileNotChildOfDir {
        parent: String,
        child: String,
    },

    #[error("General file system error: {0}")]
    FsError(String),

    #[error("General conundrum error: {0}")]
    ConundrumError(ConundrumErrorVariant),

    #[error("Conundrum could not serialize a path.")]
    PathSerializationError,

    #[error("Unsupported file extension: {0}.")]
    UnsupportedFileExtension(String),

    #[error("The provided path can't be found on your system: {0}.")]
    PathDoesntExist(String),

    #[error("Conundrum couldn't parse the file extension of the file at `{target_file}`.")]
    NoFileExtensionFound {
        target_file: String,
    },
    #[error("The extension of the file at `{target_file}` doesn't match the extensions required for the task.")]
    InvalidExtension {
        target_file: String,
    },

    #[error("Conundrum was unable to parse the file metadata at `{target_file}`.")]
    InvalidFileMeta {
        target_file: String,
    },
}

pub type ConundrumFSResult<T> where T: Sized
= Result<T, ConundrumFSError>;

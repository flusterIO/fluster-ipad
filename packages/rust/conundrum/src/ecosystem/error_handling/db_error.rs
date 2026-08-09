use serde::{Deserialize, Serialize};
use thiserror::Error;
// use conundrum_fs

use crate::{
    ecosystem::{db::tables::DatabaseTable, error_handling::conundrum_fs_error::ConundrumFSError},
    lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant,
};

#[derive(Debug, Error, Serialize, Deserialize, Clone, specta::Type)]
#[serde(tag = "tag", content = "content")]
pub enum DatabaseError {
    #[error("Conundrum Error: {:?}.", .0)]
    ConundrumError(ConundrumErrorVariant),
    #[error("Feature not yet implemented.")]
    NotImplemented,
    #[error("The database encountered a thread related error.")]
    ThreadError,
    #[error("The data directory for your operating system could not be found. We don't know where to put your data.")]
    InvalidDataDirectory,
    #[error("Conundrum encountered an error serializing some of your configuration.")]
    SerializationError,
    #[error("Fluster could not locate your operating system's data directory. We don't know where to store your database.")]
    FailToFindDataDirectory,
    #[error("Conundrum could not connect to the database.")]
    FailToConnect,
    #[error("Conundrum could not create an `{0}` entity.")]
    FailToCreateEntity(String),
    #[error("Conundrum could not save a {:?}.", .0.to_model_name())]
    FailToInsert(DatabaseTable),
    #[error("Conundrum could not delete a {:?} entity.", .0.to_model_name())]
    FailToDelete(DatabaseTable),
    #[error("Failed to serialize. Nested Error: {0}")]
    FailToSerialize(String),
    #[error("Failed to create table for the `{:?}` model.", .0.to_model_name())]
    FailToCreateTable(DatabaseTable),
    #[error("File system error: {:?}", .0)]
    FileSystemError(ConundrumFSError),
    #[error("Database Error: Duplicate identifiers")]
    DuplicateEntities,
    #[error("Failed to query the `{:?}` entity using the following predicate: `{}`", .table.to_model_name(), .predicate.as_ref().cloned().unwrap_or_default())]
    FailToQueryEntity {
        predicate: Option<String>,
        table: DatabaseTable,
    },
    #[error("The search parameters provided were empty. We don't know what to look for.")]
    EmptySearchParams,
    #[error("Invalid pagination parameters.")]
    InvalidPagination,
    #[error("The root to one of your workspaces could not be found. We attempted to look in `{0}`.")]
    InvalidWorkspacePath(String),
    #[error("Conundrum encountered an invalid date-time.")]
    InvalidDateTime,
}

impl From<ConundrumFSError> for DatabaseError {
    fn from(value: ConundrumFSError) -> Self {
        Self::FileSystemError(value)
    }
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;

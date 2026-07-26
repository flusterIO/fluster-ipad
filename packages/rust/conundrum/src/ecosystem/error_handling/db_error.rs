use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::ecosystem::db::tables::DatabaseTable;

#[derive(Debug, Error, Serialize, Deserialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum DatabaseError {
    #[error("Feature not yet implemented.")]
    NotImplemented,
    #[error("The data directory for your operating system could not be found. We don't know where to put your data.")]
    InvalidDataDirectory,
    #[error("Conundrum encountered an error serializing some of your configuration.")]
    SerializationError,
    #[error("Fluster could not locate your operating system's data directory. We don't know where to store your database.")]
    FailToFindDataDirectory,
    #[error("Conundrum could not connect to the database.")]
    FailToConnect,
    #[error("Conundrum could not save a {:?}.", .0.to_model_name())]
    FailToInsert(DatabaseTable),
    #[error("Conundrum could not delete a {:?} entity.", .0.to_model_name())]
    FailToDelete(DatabaseTable),
    #[error("Failed to serialize. Nested Error: {0}")]
    FailToSerialize(String),
    #[error("Failed to create table for the `{:?}` model.", .0.to_model_name())]
    FailToCreateTable(DatabaseTable),
}

pub type DatabaseResult<T> = Result<T, DatabaseError>;

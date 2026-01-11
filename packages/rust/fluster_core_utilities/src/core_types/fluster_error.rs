use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

#[derive(Debug, Error, uniffi::Error, Serialize, Deserialize, Type)]
pub enum FlusterError {
    #[error("A test error used for development.")]
    CanaryError,
    // -- Serialization Errors --
    #[error("Json serialization error.")]
    JsonSerializationError,
    #[error("Frontmatter error.")]
    FrontMatterError,
    #[error("Could not successfully parse this mdx content.")]
    MdxParsingError,
    // -- File System --
    #[error("Failed to read file system path at {0}.")]
    FailToReadFileSystemPath(String),
    #[error("Failed to save file at {0}.")]
    FailToSaveFile(String), //
    #[error("Could not find the data directory for your operating system. We cannot continue.")]
    DataDirNotFound,
    // -- Database --
    #[error("Could not locate data directory.")]
    FailToFindDataDirectory,
    #[error("Could not connect to the local Fluster database.")]
    FailToConnect,
    #[error("Fluster failed to create a necessary table.")]
    FailToCreateTable,
    #[error("Flusted failed to open a database table.")]
    FailToOpenTable,
    #[error("Fluster failed to start the database server. This error may be harmless.")]
    FailToStartDb,
    #[error("Fail to drop table.")]
    FailToDropTable,
    #[error("Fluster failed to insert an item into your database.")]
    FailToCreateEntity,
    #[error("Fluster failed to find what it was looking for.")]
    FailToFind,
    #[error(
        "Fluster failed to find something by a specific d. If you feel this is an issue with Fluster, please submit an issue on our github page."
    )]
    FailToFindById,
    #[error("Failed to delete an entity.")]
    FailToDelete,
}

pub type FlusterResult<T> = Result<T, FlusterError>;

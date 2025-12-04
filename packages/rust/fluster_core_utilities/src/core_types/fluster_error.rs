// #[derive(uniffi::Error, Debug)]
// pub enum FlusterError {
//     CanaryError,
//     // IndexOutOfBounds { index: u32, size: u32 },
//     // tuple-enums work.
//     // Generic(String),
// }

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum FlusterError {
    #[error("A test error used for development.")]
    CanaryError,
    // -- General Errors --
    #[error("Json serialization error.")]
    JsonSerializationError,
    #[error("Frontmatter error.")]
    FrontMatterError,
}

pub type FlusterResult<T> = Result<T, FlusterError>;

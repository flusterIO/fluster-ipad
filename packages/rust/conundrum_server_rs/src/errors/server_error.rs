use axum::{extract::Json, http::StatusCode};

#[derive(Debug, thiserror::Error, Clone)]
pub enum ServerError {
    #[error("Not Implemented")]
    NotImplemented,
    #[error("I'll fill this out later... it's just docgen stuff.")]
    GeneralError,
    #[error("Latex conversion error.")]
    LatexConversionError,
}

pub type ServerResult<T> = Result<T, ServerError>;
pub type ServerResultResponseEmpty = Result<(), StatusCode>;
pub type ServerResultResponse<T> = Result<Json<T>, StatusCode>;

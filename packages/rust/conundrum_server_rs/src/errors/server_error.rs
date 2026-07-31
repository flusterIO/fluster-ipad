use axum::{extract::Json, http::StatusCode};
use specta::Type;

#[derive(Debug, thiserror::Error, Clone, Type)]
pub enum ServerError {
    #[error("Not Implemented")]
    NotImplemented,
    #[error("I'll fill this out later... it's just docgen stuff.")]
    GeneralError,
    #[error("Latex conversion error.")]
    LatexConversionError,
    #[error("Conundrum could not render html properly.")]
    HtmlRenderError,
}

impl Into<StatusCode> for ServerError {
    fn into(self) -> StatusCode {
        match self {
            Self::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            Self::HtmlRenderError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::LatexConversionError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::GeneralError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub type ServerResult<T> = Result<T, ServerError>;
pub type ServerResultResponseEmpty = Result<(), StatusCode>;
pub type ServerResultResponseHtml = Result<String, StatusCode>;
pub type ServerResultResponseJson<T> = Result<Json<T>, StatusCode>;

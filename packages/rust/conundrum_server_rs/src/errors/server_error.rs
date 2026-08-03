use axum::{extract::Json, http::StatusCode};
use conundrum::ecosystem::error_handling::db_error::DatabaseError;
use rspc::{Error, ResolverError};
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
    #[error("Not Found.")]
    NotFound,
    #[error("Database Error: {:?}", .0)]
    DatabaseError(DatabaseError),
    #[error("The server encounterd an error that it cannot recover from: {:?}", .0)]
    CoreFailure(String),
}

impl Error for ServerError {
    fn into_procedure_error(self) -> rspc::ProcedureError {
        match self {
            Self::NotFound => rspc::ProcedureError::NotFound,
            Self::NotImplemented => {
                rspc::ProcedureError::Resolver(ResolverError::new::<_, ServerError>("Method not yet implemented", None))
            }
            Self::HtmlRenderError => {
                rspc::ProcedureError::Resolver(ResolverError::new::<_, ServerError>("Failed attempting to render html.",
                                                                                    None))
            }
            Self::GeneralError => {
                rspc::ProcedureError::Resolver(ResolverError::new::<_, ServerError>("Method not yet implemented", None))
            }
            Self::LatexConversionError => {
                rspc::ProcedureError::Resolver(ResolverError::new::<_, ServerError>("Latex conversion error.", None))
            }
            Self::DatabaseError(e) => {
                rspc::ProcedureError::Resolver(ResolverError::new::<_, ServerError>(format!("Database Error: {:?}", e),
                                                                                    None))
            }
            Self::CoreFailure(e) => {
                rspc::ProcedureError::Resolver(ResolverError::new::<_, ServerError>(format!("{:?}", e), None))
            }
        }
    }
}

impl Into<StatusCode> for ServerError {
    fn into(self) -> StatusCode {
        match self {
            Self::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            Self::HtmlRenderError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::LatexConversionError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::GeneralError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::CoreFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub type ServerResult<T> = Result<T, ServerError>;
pub type ServerResultResponseEmpty = Result<(), StatusCode>;
pub type ServerResultResponseHtml = Result<String, StatusCode>;
pub type ServerResultResponseJson<T> = Result<Json<T>, StatusCode>;

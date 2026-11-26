use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

/// Request-level failures. Per-file problems are not errors; they are
/// reported inside successful responses as [`super::models::FileIssue`]s so
/// one bad file never fails a whole sync.
#[derive(Debug, thiserror::Error)]
pub enum FsSyncError {
    #[error("The sync root `{0}` does not exist or is not a directory.")]
    InvalidRoot(String),
    #[error("{requested} paths were requested, but at most {max} may be requested at once.")]
    TooManyPaths { requested: usize, max: usize },
    #[error("The file system sync failed unexpectedly: {0}")]
    Internal(String),
}

impl FsSyncError {
    fn code(&self) -> &'static str {
        match self {
            FsSyncError::InvalidRoot(_) => "invalid_root",
            FsSyncError::TooManyPaths { .. } => "too_many_paths",
            FsSyncError::Internal(_) => "internal",
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            FsSyncError::InvalidRoot(_) => StatusCode::UNPROCESSABLE_ENTITY,
            FsSyncError::TooManyPaths { .. } => StatusCode::PAYLOAD_TOO_LARGE,
            FsSyncError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Serialize)]
struct ErrorEnvelope {
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: &'static str,
    message: String,
}

impl IntoResponse for FsSyncError {
    fn into_response(self) -> Response {
        if let FsSyncError::Internal(_) = &self {
            log::error!("{}", self);
        }
        let body = ErrorEnvelope { error: ErrorDetail { code: self.code(),
                                                        message: self.to_string() } };
        (self.status(), Json(body)).into_response()
    }
}

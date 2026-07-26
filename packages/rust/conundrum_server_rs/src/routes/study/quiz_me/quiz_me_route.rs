use axum::http::StatusCode;

use crate::errors::server_error::ServerResultResponseEmpty;

pub async fn quiz_me_route() -> ServerResultResponseEmpty {
    Err(StatusCode::NOT_IMPLEMENTED)
}

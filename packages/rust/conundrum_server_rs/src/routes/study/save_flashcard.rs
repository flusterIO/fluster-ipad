use axum::{extract::Json, http::StatusCode};
use serde::Deserialize;

use crate::errors::server_error::ServerResultResponseEmpty;

#[derive(Clone, Deserialize)]
pub struct SaveFlashcardRequestData {
    pub question: String,
    pub answer: String,
}

pub async fn save_flashcard_route(Json(req): Json<SaveFlashcardRequestData>) -> ServerResultResponseEmpty {
    Err(StatusCode::NOT_IMPLEMENTED)
}

use std::sync::Arc;

use axum::{Router, routing::post};
use conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState;

use crate::rest::fs_sync::handlers;

pub fn get_fs_sync_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new().route("/manifest", post(handlers::manifest))
                                     .route("/contents", post(handlers::contents))
}

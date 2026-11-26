//! Composition root for the plain JSON REST API served under `/api/v1`.
//!
//! REST is the surface for native clients (Swift, etc.) that don't consume the
//! rspc TypeScript bindings. Add a feature by creating a module under
//! `src/rest/`, exposing a `get_<feature>_router()`, and nesting it here.
use std::sync::Arc;

use axum::{Json, Router, routing::get};
use conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState;

use crate::rest::fs_sync::{
    models::{FS_SYNC_PROTOCOL_VERSION, HealthResponse},
    router::get_fs_sync_router,
};

pub const REST_API_PREFIX: &str = "/api/v1";

pub fn get_rest_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new().route("/health", get(health))
                                     .nest("/fs_sync", get_fs_sync_router())
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok".to_string(),
                          server_version: env!("CARGO_PKG_VERSION").to_string(),
                          fs_sync_protocol_version: FS_SYNC_PROTOCOL_VERSION })
}

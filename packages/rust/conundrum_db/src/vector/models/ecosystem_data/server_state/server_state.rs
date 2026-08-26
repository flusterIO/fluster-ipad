use std::{sync::Arc, time::Duration};

use axum::extract::FromRef;
use lancedb::Connection;
use rust_mcp_sdk::{
    ToMcpServerHandler, TransportOptions,
    id_generator::{FastIdGenerator, UuidGenerator},
    mcp_http::McpAppState,
    mcp_server::ServerHandler,
    schema::{Implementation, InitializeResult, ProtocolVersion, ServerCapabilities, ServerCapabilitiesTools},
    session_store::InMemorySessionStore,
};

use conundrum::{
    ai::rig::{rig_client_local::RigClientLocal, rig_client_remote::RigClientRemote},
    ecosystem::{
        db::db::get_database,
        error_handling::db_error::{DatabaseError, DatabaseResult},
    },
    lang::lib::shared::utility_types::ArcTokioMutex,
};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ServerState {
    pub db: ArcTokioMutex<Connection>,
    pub local_client: Option<ArcTokioMutex<RigClientLocal>>,
    pub remote_client: Option<ArcTokioMutex<RigClientRemote>>,
}

impl ServerState {
    pub async fn try_new() -> DatabaseResult<Self> {
        let db = get_database().await.map_err(|e| DatabaseError::FailToConnect)?;
        let local_client = RigClientLocal::initialize()
            .inspect_err(|_| {
                log::warn!("Failed to load a valid Ollama environment. Cannot continue with certain local AI actions.")
            }).ok().map(|x| {
                Arc::new(Mutex::new(x))
            });

        let remote_client = RigClientRemote::initialize()
            .inspect_err(|_| {
                log::warn!("Failed to load a valid remote AI environment. Cannot continue with certain server scale AI actions.")
            }).ok().map(|x| {
                Arc::new(Mutex::new(x))
            });

        Ok(Self { db,
                  local_client,
                  remote_client })
    }
}

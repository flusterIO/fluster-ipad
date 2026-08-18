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
    // pub remote_agent: Option<Agent>
    pub mcp: McpAppState,
}

impl ServerState {
    pub async fn try_new(handler: impl ServerHandler) -> DatabaseResult<Self> {
        let db = get_database().await.map_err(|e| DatabaseError::FailToConnect)?;

        let server_info =
            InitializeResult { server_info: Implementation { name: "conundrum-mcp".into(),
                                                             version: "0.0.1".into(),
                                                             title: Some("Conundrum MCP Server".into()),
                                                             description:
                                                                 Some("A modular academic toolkit.".into()),
                                                             icons: vec![],
                                                             website_url: Some("https://flusterapp.com".into()) },
                               capabilities: ServerCapabilities { tools:
                                                                      Some(ServerCapabilitiesTools { list_changed:
                                                                                                         None }),
                                                                  ..Default::default() },
                               protocol_version: ProtocolVersion::V2025_11_25.into(),
                               instructions: None,
                               meta: None };
        let mcp = McpAppState { session_store: Arc::new(InMemorySessionStore::new()),
                                id_generator: Arc::new(UuidGenerator {}),
                                server_details: Arc::new(server_info),
                                handler: handler.to_mcp_server_handler(),
                                stream_id_gen: Arc::new(FastIdGenerator::new("cdrm".into())),
                                ping_interval: Duration::new(30, 0),
                                task_store: None,
                                client_task_store: None,
                                message_observer: None,
                                event_store: None,
                                enable_json_response: true,
                                transport_options: Arc::new(TransportOptions::default()) };
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
                  mcp,
                  local_client,
                  remote_client })
    }
}

impl FromRef<ServerState> for McpAppState {
    fn from_ref(input: &ServerState) -> Self {
        input.mcp.clone()
    }
}

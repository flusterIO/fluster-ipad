use std::{sync::Arc, time::Duration};

use axum::extract::FromRef;
use conundrum_db::vector::{
    database::db::{ArcMutexDB, CdrmDb, get_database},
    models::{
        ai::{agent::agent_description::AgentDescription, tool::mcp_tool_name_list::MCPToolNameList},
        date_time::date_time::DateTime,
        primitives::db_id::DatabaseId,
    },
};
use rig::completion::CompletionModel;
use rust_mcp_sdk::{
    ToMcpServerHandler, TransportOptions,
    id_generator::{FastIdGenerator, UuidGenerator},
    mcp_http::McpAppState,
    schema::{Implementation, InitializeResult, ProtocolVersion, ServerCapabilities, ServerCapabilitiesTools},
    session_store::InMemorySessionStore,
};

use crate::{
    errors::server_error::{ServerError, ServerResult},
    mcp::mcp_handler::ConundrumMCP,
    rig::{
        agents::local_agent::LocalAgent, ai_traits::from_agent_description::FromAgentDescription,
        rig_client_local::RigClientLocal,
    },
};

#[derive(Clone)]
pub struct ServerState {
    pub db: ArcMutexDB,
    pub local_agent: Option<LocalAgent>,
    // pub remote_agent: Option<Agent>
    pub mcp: McpAppState,
}

impl ServerState {
    pub async fn try_new() -> ServerResult<Self> {
        let db = get_database().await.map_err(|e| ServerError::DatabaseError(e))?;

        let server_info =
            InitializeResult { server_info: Implementation { name: "conundrum-mcp".into(),
                                                             version: "0.0.1".into(),
                                                             title: Some("Conundrum MCP Server".into()),
                                                             description:
                                                                 Some("An all-in-one academic toolkit.".into()),
                                                             icons: vec![],
                                                             website_url: Some("https://flusterapp.com".into()) },
                               capabilities: ServerCapabilities { tools:
                                                                      Some(ServerCapabilitiesTools { list_changed:
                                                                                                         None }),
                                                                  ..Default::default() },
                               protocol_version: ProtocolVersion::V2025_11_25.into(),
                               instructions: None,
                               meta: None };
        let handler = ConundrumMCP::default();
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
        let local_agent = LocalAgent::from_agent_description(AgentDescription { id: DatabaseId::new(),
                                                                                model: "qwen3:8b".to_string(),
                                                                                name: None,
                                                                                temperature_scalar: 0.9,
                                                                                instructions: None,
                                                                                primary_task: None,
                                                                                is_local: true,
                                                                                always_include_tools:
                                                                                    MCPToolNameList::new_empty(),
                                                                                ctime: DateTime::new_now(),
                                                                                utime: DateTime::new_now() })?;
        Ok(Self { db,
                  mcp,
                  local_agent: Some(local_agent) })
    }
}

// impl FromRef<ServerState> for Arc<McpAppState> {
//     fn from_ref(input: &ServerState) -> Self {
//         input.mcp.clone()
//     }
// }

impl FromRef<ServerState> for McpAppState {
    fn from_ref(input: &ServerState) -> Self {
        input.mcp.clone()
    }
}

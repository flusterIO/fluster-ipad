use std::{sync::Arc, time::Duration};

use axum::Router;
use rust_mcp_axum::{McpMountOptions, mcp_routes};
use rust_mcp_sdk::{
    ToMcpServerHandler, TransportOptions,
    id_generator::{FastIdGenerator, UuidGenerator},
    mcp_http::{McpAppState, McpHttpHandler},
    schema::{Implementation, InitializeResult, ProtocolVersion, ServerCapabilities, ServerCapabilitiesTools},
    session_store::InMemorySessionStore,
};

use crate::{errors::server_error::ServerResult, mcp::mcp_handler::ConundrumMCP};

pub fn get_mcp_server() -> ServerResult<Router<()>> {
    let server_info =
        InitializeResult { server_info: Implementation { name: "conundrum-mcp".into(),
                                                         version: "0.0.1".into(),
                                                         title: Some("Conundrum MCP Server".into()),
                                                         description:
                                                             Some("An all-in-one academic toolkit.".into()),
                                                         icons: vec![],
                                                         website_url: Some("https://flusterapp.com".into()) },
                           capabilities: ServerCapabilities { tools:
                                                                  Some(ServerCapabilitiesTools { list_changed: None }),
                                                              ..Default::default() },
                           protocol_version: ProtocolVersion::V2025_11_25.into(),
                           instructions: None,
                           meta: None };
    let handler = ConundrumMCP::default();
    let state = Arc::new(McpAppState { session_store: Arc::new(InMemorySessionStore::new()),
                                       id_generator: Arc::new(UuidGenerator {}),
                                       server_details: Arc::new(server_info),
                                       handler: handler.to_mcp_server_handler(),
                                       stream_id_gen: Arc::new(FastIdGenerator::new("cdrm".into())),
                                       ping_interval: Duration::new(180, 0),
                                       task_store: None,
                                       client_task_store: None,
                                       message_observer: None,
                                       event_store: None,
                                       enable_json_response: true,
                                       transport_options: Arc::new(TransportOptions::default()) });
    let mount = McpMountOptions { streamable_http_endpoint: "/mcp".into(),
                                  sse_endpoint: "/sse".into(),
                                  sse_messages_endpoint: "/messages".into(),
                                  health_endpoint: Some("/health".into()),
                                  ..Default::default() };
    let http_handler = McpHttpHandler::new(None, vec![], None);
    let x = mcp_routes(state, &mount, http_handler);
    Ok(x)
}

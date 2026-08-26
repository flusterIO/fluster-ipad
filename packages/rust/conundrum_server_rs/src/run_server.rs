use crate::mcp::mcp_handler::ConundrumMCP;
use crate::mcp::mcp_server::get_mcp_server;
use crate::mcp::server::ConundrumMcpServer;
use crate::rest::handle_socket::handle_socket;
pub use crate::rpc::rspc_router::get_rspc_router;
use axum::extract::State;
use axum::{Router, extract::WebSocketUpgrade, response::Response, routing::get};
use conundrum::ecosystem::environment_variables::cdrm_env_variable::{CdrmEnvVariable, DEFAULT_CDRM_SERVER_PORT};
use conundrum::ecosystem::error_handling::server_error::{ServerError, ServerResult};
use conundrum_db::vector::models::ecosystem_data::server_state::server_state::ServerState;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};
#[cfg(debug_assertions)]
use rspc::Typescript;
pub use rspc_axum;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub fn get_server_port() -> u32 {
    match CdrmEnvVariable::ServerPort.read() {
        Ok(c) => {
            let n: Result<u32, _> = c.parse();
            n.unwrap_or(*DEFAULT_CDRM_SERVER_PORT)
        }
        Err(_) => {
            log::warn!("Failed to load the `CDRM_SERVER_PORT` environment variable. Falling back to the default: {}",
                       DEFAULT_CDRM_SERVER_PORT);
            *DEFAULT_CDRM_SERVER_PORT
        }
    }
}

pub async fn run_server(write_types_to: Option<impl AsRef<std::path::Path>>) -> ServerResult<()> {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let (rpc_router, types) =
        get_rspc_router().await
                         .expect("Failed to generate rspc router. This is a major issue that can't be recovered from.");

    #[cfg(debug_assertions)]
    {
        if let Some(fp) = write_types_to {
            Typescript::default().export_to(fp, &types)
                                 .inspect_err(|e| {
                                     log::error!("Codegen Error: {:?}", e);
                                 })
                                 .expect("Failed to compile rpc types");
        }
    }

    let cancellation_token = tokio_util::sync::CancellationToken::new();

    let mcp_service =
        StreamableHttpService::new(
            || {
                Ok(ConundrumMcpServer::default())
            },
            LocalSessionManager::default().into(),
            StreamableHttpServerConfig::default()
                .with_cancellation_token(
                    cancellation_token.child_token()
                ),
        );

    let state = ServerState::try_new().await.map(Arc::new).expect("We cannot establish a connection to the database, which is odd, because it's embedded. Have you ran the initialize command? Try running `cdrm initialize-database` if you have the cdrm cli installed.");
    let cloned_state = Arc::clone(&state);
    let app =
        Router::<Arc<ServerState>>::new().route("/api/ws", get(ws_handler))
                                         .nest_service("/api/rpc",
                                                       rspc_axum::endpoint(rpc_router, move || {
                                                           Arc::clone(&cloned_state)
                                                       }).layer(cors))
                                         .nest_service("/api/mcp", mcp_service)
                                         .with_state(Arc::clone(&state));
    let port = get_server_port();
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

    axum::serve(listener, app).with_graceful_shutdown(async move {
                                  tokio::signal::ctrl_c().await.expect("failed to listen for Ctrl+C");
                                  cancellation_token.cancel();
                              })
                              .await
                              .map_err(|e| {
                                  log::error!("Error: {}", e);
                                  ServerError::CoreFailure("Dude I don't even know...".to_string())
                              })?;
    Ok(())
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<ServerState>>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

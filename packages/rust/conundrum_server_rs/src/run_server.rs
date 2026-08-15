use std::sync::Arc;

pub use crate::rpc::rspc_router::get_rspc_router;
use crate::{
    errors::server_error::{ServerError, ServerResult},
    rig::ai_types::ai_types::LocalCompletionModel,
    server_state::ServerState,
};
use axum::extract::State;
#[cfg(debug_assertions)]
use rspc::Typescript;
pub use rspc_axum;
use tower_http::cors::{Any, CorsLayer};

use crate::mcp::mcp_server::get_mcp_server;
use crate::rest::handle_socket::handle_socket;
use axum::{Router, extract::WebSocketUpgrade, response::Response, routing::get};
use conundrum::ecosystem::environment_variables::cdrm_env_variable::{CdrmEnvVariable, DEFAULT_CDRM_SERVER_PORT};

pub fn get_server_port() -> u32 {
    match CdrmEnvVariable::ServerPort.read() {
        Ok(c) => {
            let n: Result<u32, _> = c.parse();
            n.unwrap_or(*DEFAULT_CDRM_SERVER_PORT)
        }
        Err(err) => {
            log::warn!("Error: {}", err);
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

    let state = ServerState::try_new().await.map(Arc::new).expect("We cannot establish a connection to the database, which is odd, because it's embedded. Have you ran the initialize command? Try running `cdrm initialize-database` if you have the cdrm cli installed.");
    // TODO: Move the context up to a shared context and add a database connection.
    //
    let cloned_state = Arc::clone(&state);

    let mcp_router = get_mcp_server().expect("Failed to generate the MCP server. Cannot continue.");
    let app = Router::<Arc<ServerState>>::new().route("/api/ws", get(ws_handler))
                                               .nest_service("/api/rpc",
                                                             rspc_axum::endpoint(rpc_router, move || {
                                                                 let y = Arc::clone(&cloned_state);
                                                                 y
                                                             }).layer(cors))
                                               .nest_service("/api/mpc", mcp_router)
                                               .with_state(Arc::clone(&state));
    let port = get_server_port();
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

    axum::serve(listener, app).await.map_err(|e| {
                                         log::error!("Error: {}", e);
                                         ServerError::CoreFailure("Dude I don't even know...".to_string())
                                     })?;
    Ok(())
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<ServerState>>) -> Response {
    ws.on_upgrade(move |socket| handle_socket::<LocalCompletionModel>(socket, state))
}

pub use crate::rpc::rspc_router::get_rspc_router;
#[cfg(debug_assertions)]
use rspc::Typescript;
pub use rspc_axum;
use tower_http::cors::{Any, CorsLayer};

use crate::{mcp::mcp_server::get_mcp_server, rpc::route_context::RouteContext};
use axum::Router;
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

pub async fn run_server(write_types_to: Option<impl AsRef<std::path::Path>>) {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let (rpc_router, types) =
        get_rspc_router().await
                         .expect("Failed to generate rspc router. This is a major issue that can't be recovered from.");

    #[cfg(debug_assertions)] // Only export in development builds
    {
        if let Some(fp) = write_types_to {
            Typescript::default().export_to(fp, &types)
                                 .inspect_err(|e| {
                                     println!("Codegen Error: {:?}", e);
                                 })
                                 .expect("Failed to compile rpc types");
        }
    }

    let state = RouteContext::try_new().await.expect("We cannot establish a connection to the database, which is odd, because it's embedded. Have you ran the initialize command? Try running `cdrm initialize-database` if you have the cdrm cli installed.");
    // TODO: Move the context up to a shared context and add a database connection.

    let mcp_router = get_mcp_server().expect("Failed to generate the MCP server. Cannot continue.");
    let app = Router::new().nest_service("/rpc", rspc_axum::endpoint(rpc_router, move || state.clone()).layer(cors))
                           .merge(mcp_router);
    let port = get_server_port();
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

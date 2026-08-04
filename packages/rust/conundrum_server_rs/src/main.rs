pub mod errors;
pub mod routes;
pub mod rpc;
#[cfg(debug_assertions)]
use std::path::PathBuf;

pub use crate::rpc::rspc_router::get_rspc_router;
#[cfg(debug_assertions)]
use rspc::Typescript;
pub use rspc_axum;
use tower_http::cors::{Any, CorsLayer};

use axum::{Router, extract::State};
use conundrum::ecosystem::environment_variables::cdrm_env_variable::{CdrmEnvVariable, DEFAULT_CDRM_SERVER_PORT};

use crate::rpc::route_context::RouteContext;

#[tokio::main]
pub async fn main() {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let (rpc_router, types) =
        get_rspc_router().await
                         .expect("Failed to generate rspc router. This is a major issue that can't be recovered from.");

    #[cfg(debug_assertions)] // Only export in development builds
    {
        Typescript::default()
        .export_to(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../conundrum_frontend/src/core/codegen/bindings.ts"), &types)
        .inspect_err(|e| {
            println!("Codegen Error: {:?}", e);
        }).expect("Failed to compile rpc types");
    }

    let state = RouteContext::try_new().await.expect("We cannot establish a connection to the database, which is odd, because it's embedded. Have you ran the initialize command? Try running `cdrm initialize-database` if you have the cdrm cli installed.");
    // TODO: Move the context up to a shared context and add a database connection.

    let app = Router::new().nest_service("/rpc", rspc_axum::endpoint(rpc_router, move || state.clone()).layer(cors));
    let port = match CdrmEnvVariable::ServerPort.read() {
        Ok(c) => {
            let n: Result<u32, _> = c.parse();
            n.unwrap_or(*DEFAULT_CDRM_SERVER_PORT)
        }
        Err(err) => {
            log::warn!("Error: {}", err);
            *DEFAULT_CDRM_SERVER_PORT
        }
    };
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

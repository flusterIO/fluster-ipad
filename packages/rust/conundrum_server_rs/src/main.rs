pub mod errors;
pub mod routes;
pub mod rpc;
pub use crate::rpc::rspc_router::get_rspc_router;
pub use rspc_axum;
use tower_http::cors::{Any, CorsLayer};

use axum::Router;
use conundrum::ecosystem::environment_variables::cdrm_env_variable::{CdrmEnvVariable, DEFAULT_CDRM_SERVER_PORT};

use crate::{
    routes::{
        route_enum::RouteEnum,
        study::{
            quiz_me::quiz_me_route::quiz_me_route, random_question::get_random_question,
            save_flashcard::save_flashcard_route,
        },
    },
    rpc::route_context::RouteContext,
};

#[tokio::main]
pub async fn main() {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);
    let (rpc_router, _) = get_rspc_router();

    // TODO: Move the context up to a shared context and add a database connection.

    let app = Router::new().nest_service("/rpc",
                                         rspc_axum::endpoint::<RouteContext, _, _, _>(rpc_router, || {
                                             RouteContext::default()
                                         }).layer(cors));
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

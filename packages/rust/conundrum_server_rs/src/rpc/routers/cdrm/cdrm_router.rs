use std::sync::Arc;

use crate::routes::fs::route_exists::path_exists;
use conundrum::ecosystem::error_handling::server_error::{ServerError, ServerResult};
use conundrum::{
    ecosystem::db::db_traits::async_traits::actionable_request::ActionableRequest,
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};
use conundrum_db::vector::{
    models::ecosystem_data::server_state::server_state::ServerState,
    parameters::cdrm::compile_cdrm_request::CompileCdrmRequest,
};
use rspc::{Procedure, Router};

pub fn get_cdrm_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new().procedure("compile_cdrm",
                                            Procedure::<Arc<ServerState>, CompileCdrmRequest, MdxParsingResult>::builder::<ServerError>().mutation(|_, req: CompileCdrmRequest| async move {
                                                req.execute_request().await
                                                    .map_err(|e| {
                                                        ServerError::DatabaseError(e)
                                                    })
                                                                               }))
}

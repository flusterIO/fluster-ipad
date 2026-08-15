use std::sync::Arc;

use crate::server_state::ServerState;
use crate::{
    errors::server_error::{ServerError, ServerResult},
    routes::fs::route_exists::path_exists,
};
use conundrum::output::parsing_result::mdx_parsing_result::MdxParsingResult;
use conundrum_db::vector::{
    database::db_traits::async_traits::actionable_request::ActionableRequest,
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

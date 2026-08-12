use conundrum::output::parsing_result::mdx_parsing_result::MdxParsingResult;
use crate::{
    errors::server_error::{ServerError, ServerResult},
    routes::fs::route_exists::path_exists,
    rpc::route_context::RouteContext,
};
use conundrum_db::vector::{
    database::db_traits::async_traits::actionable_request::ActionableRequest,
    parameters::cdrm::compile_cdrm_request::CompileCdrmRequest
};
use rspc::{Procedure, Router};

pub fn get_cdrm_router() -> Router<RouteContext> {
    Router::<RouteContext>::new().procedure("compile_cdrm",
                                            Procedure::<RouteContext, CompileCdrmRequest, MdxParsingResult>::builder::<ServerError>().mutation(|_, req: CompileCdrmRequest| async move {
                                                req.execute_request().await
                                                    .map_err(|e| {
                                                        ServerError::DatabaseError(e)
                                                    })
                                                                               }))
}

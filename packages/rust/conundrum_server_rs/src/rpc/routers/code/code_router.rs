use std::sync::Arc;

use crate::routes::fs::route_exists::path_exists;
use conundrum::ecosystem::db::db_traits::async_traits::actionable_request::ActionableRequest;
use conundrum::ecosystem::error_handling::server_error::{ServerError, ServerResult};
use conundrum_db::vector::{
    models::ecosystem_data::server_state::server_state::ServerState,
    parameters::code::highlight_code_request::HighlightCodeRequest,
};
use rspc::{Procedure, Router};

pub fn get_code_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new().procedure("highlight_code",
                                            Procedure::<Arc<ServerState>, HighlightCodeRequest, String>::builder::<ServerError>().mutation(|_, req: HighlightCodeRequest| async move {
                                                let res = req.execute_request().await?;
                                                                                   Ok(res)
                                                                               }))
}

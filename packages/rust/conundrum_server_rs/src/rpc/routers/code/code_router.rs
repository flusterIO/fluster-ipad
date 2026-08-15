use std::sync::Arc;

use crate::server_state::ServerState;
use crate::{
    errors::server_error::{ServerError, ServerResult},
    routes::fs::route_exists::path_exists,
};
use conundrum_db::vector::{
    database::db_traits::async_traits::actionable_request::ActionableRequest,
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

use crate::{
    errors::server_error::{ServerError, ServerResult},
    routes::fs::route_exists::path_exists,
    rpc::route_context::RouteContext,
};
use conundrum_db::vector::{
    database::db_traits::async_traits::actionable_request::ActionableRequest,
    parameters::fs::path_validity_check::PathValidationRequest,
};
use rspc::{Procedure, Router};

pub fn get_fs_router() -> Router<RouteContext> {
    Router::<RouteContext>::new()
        .procedure("validate_path",
                                            Procedure::<RouteContext, PathValidationRequest, bool>::builder::<ServerError>().query(|_, req: PathValidationRequest| async move {
                                                let x = req.execute_request().await?;
                                                                                   Ok(x)
                                                                               }))
}

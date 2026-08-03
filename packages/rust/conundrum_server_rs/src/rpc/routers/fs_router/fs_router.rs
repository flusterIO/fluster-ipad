use rspc::{Procedure, Router};

use crate::{
    errors::server_error::{ServerError, ServerResult},
    routes::fs::route_exists::path_exists,
    rpc::route_context::RouteContext,
};

pub fn get_fs_router() -> Router<RouteContext> {
    Router::<RouteContext>::new().procedure("path_exists",
                                            Procedure::<RouteContext, String, bool>::builder::<ServerError>().query(|_, path: String| async move {
                                                let x = path_exists(path.as_str()).await;
                                                                                   Ok(x)
                                                                               }))
}

use rspc::{Procedure, Router};

use crate::{
    errors::server_error::ServerError, routes::fs::route_exists::path_exists, rpc::route_context::RouteContext,
};

pub fn get_fs_router() -> Router<RouteContext> {
    Router::<RouteContext>::new().procedure("path_exists",
                                            Procedure::builder::<ServerError>().query(|_, path: String| async {
                                                                                   Ok(path_exists(path.as_str()))
                                                                               }))
}

use crate::{
    errors::server_error::{ServerError, ServerResult},
    routes::fs::route_exists::path_exists,
    rpc::{
        route_context::RouteContext,
        routers::fs::fs_path_simple_result::{FSPathSimpleResult, PathVariant},
    },
};
use conundrum::{
    ecosystem::error_handling::{conundrum_fs_error::ConundrumFSError, db_error::DatabaseError},
    lang::constants::file_types::ParsableFileType,
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
        .procedure("explore_directory",
                                            Procedure::<RouteContext, String, Vec<FSPathSimpleResult>>::builder::<ServerError>().query(|_, req: String| async move {
                                                let f = std::fs::read_dir(
                                                    req
                                                )
                                                    .map_err(|e| {
                                                        log::warn!("Directory Error: {:?}", e);
                                                        DatabaseError::FileSystemError(
                                                            ConundrumFSError::InvalidDirectory
                                                        )
                                                    })?;
                                                let mut items: Vec<FSPathSimpleResult> = Vec::new();
                                                for res in f {
                                                   if let Ok(item) = res  {
                                                       let p = item.path();
                                                       if let Some(s) = p.to_str() {
                                                       let parsable = p.clone().extension().map(|s| s.to_str()).flatten().map(|s| {
                                                           ParsableFileType::try_from(s.to_string()).ok()
                                                       }).flatten();
                                                        let is_dir = &p.is_dir();
                                                        let is_file = &p.is_file();
                                                        let is_unsure = !is_dir && !is_file;
                                                        let variant = match is_unsure {
                                                            true => PathVariant::Dir,
                                                            false => {
                                                                match is_file {
                                                                    true => PathVariant::File,
                                                                    false => PathVariant::Dir
                                                                }
                                                            }
                                                        };
                                                       let x = FSPathSimpleResult {
                                                            path: s.to_string(),
                                                            parsable,
                                                            variant,

                                                       };
                                                       items.push(x);
                                                       }
                                                   }
                                                }
                                                Ok(items)
                                                                               }))
}

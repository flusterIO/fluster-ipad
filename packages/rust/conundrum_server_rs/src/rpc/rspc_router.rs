use std::ops::Index;

use axum::extract::State;
use conundrum_db::vector::{
    database::{
        db::get_database,
        db_traits::{db_entity::DBEntity, entity_crud::EntityCRUD},
        pagination::PaginationParams,
        schema_version::{schema_version::SchemaVersion, server_version::ServerVersion},
    },
    models::{
        ecosystem_data::ecosystem_data::VersionData,
        workspace::{
            user_workspace::{self, UserWorkspace},
            user_workspace_partial::UserWorkspacePartial,
        },
    },
    parameters::predicate_query_params::PredicateQueryParams,
};
use rspc::Procedure;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
    errors::server_error::{ServerError, ServerResult},
    rpc::{
        route_context::RouteContext,
        routers::{
            fs_router::fs_router::get_fs_router,
            workspace_management::workspace_management_router::get_workspace_management_router,
        },
    },
};

pub async fn get_rspc_router() -> ServerResult<(rspc::Procedures<RouteContext>, rspc::Types)> {
    let fs_router = get_fs_router();
    // let mut study_router = get_study_router();
    let workspace_router = get_workspace_management_router();

    let user_workspace_crud_router = rspc::Router::<crate::rpc::route_context::RouteContext>::new()
    .procedure("get_by_predicate", Procedure::<crate::rpc::route_context::RouteContext, PredicateQueryParams, Vec<UserWorkspace>>::builder::<crate::errors::server_error::ServerError>().query(|state: RouteContext , params: PredicateQueryParams | async move {
        let r = UserWorkspace::get_by_predicate(params.predicate, params.pagination, &state.db).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(r)
        }))
    .procedure("save_many", Procedure::<crate::rpc::route_context::RouteContext, Vec<UserWorkspace>, ()>::builder::<crate::errors::server_error::ServerError>().query(|state: RouteContext, params: Vec<UserWorkspace> | async move {
        UserWorkspace::save_many(params, &state.db).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(())
        }))
    .procedure("update_many", Procedure::<crate::rpc::route_context::RouteContext, Vec<UserWorkspacePartial>, ()>::builder::<crate::errors::server_error::ServerError>().query(|state: RouteContext, params: Vec<UserWorkspacePartial> | async move {
        UserWorkspace::merge_by_primary_key(params, &state.db).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(())
        }))
    .procedure("delete_by_predicate", Procedure::<crate::rpc::route_context::RouteContext, String, ()>::builder::<crate::errors::server_error::ServerError>().query(|state: RouteContext, params: String | async move {
        UserWorkspace::delete_by_predicate(params.as_str(), &state.db).await.map_err(|e| {
            log::error!("Error: {:?}", e);
            ServerError::DatabaseError(e)
        })?;
        Ok(())
        }));
    let r = rspc::Router::<RouteContext>::new().nest("fs", fs_router)
                                               .nest("workspace", workspace_router)
                                               // .nest("study", study_router)
                                               .nest("user_workspace_crud", user_workspace_crud_router)
                                               .procedure("version",
                                                          Procedure::builder::<ServerError>().query(|_, _: ()| async {
                                                              Ok(VersionData { server:
                                                                                   ServerVersion::current_version(),
                                                                               database:
                                                                                   SchemaVersion::current_version() })
                                                          }))
                                               .build()
                                               .map_err(|e| {
                                                   log::error!("Error: {:?}", e);
                                                   ServerError::CoreFailure(e.index(0).to_string())
                                               })?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rspc::Typescript;

    use super::*;

    #[tokio::test]
    async fn write_rspc_route_types() {
        let (_, types) = get_rspc_router().await.expect("Compiles rspc router.");
        Typescript::default()
        .enable_source_maps()
        .export_to(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../conundrum_frontend/src/core/codegen/bindings.ts"), &types);
    }
}

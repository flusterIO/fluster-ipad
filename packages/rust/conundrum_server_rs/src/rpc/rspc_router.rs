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
    crud_router,
    errors::server_error::{ServerError, ServerResult},
    rpc::{
        route_context::RouteContext,
        routers::{
            cdrm::cdrm_router::get_cdrm_router,
            code::code_router::get_code_router,
            describe::describe_router::{self, get_describe_router},
            fs::fs_router::get_fs_router,
            workspace_management::workspace_management_router::get_workspace_management_router,
        },
    },
};

pub async fn get_rspc_router() -> ServerResult<(rspc::Procedures<RouteContext>, rspc::Types)> {
    let fs_router = get_fs_router();
    // let mut study_router = get_study_router();
    let workspace_router = get_workspace_management_router();
    let code_router = get_code_router();
    let cdrm_router = get_cdrm_router();
    let describe_router = get_describe_router();

    let user_workspace_crud_router = crud_router!(UserWorkspace, UserWorkspacePartial);

    let r = rspc::Router::<RouteContext>::new().nest("fs", fs_router)
                                               .nest("workspace", workspace_router)
                                               .nest("code", code_router)
                                               .nest("cdrm", cdrm_router)
                                               .nest("describe", describe_router)
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
        Typescript::default().enable_source_maps()
        .export_to(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../conundrum_frontend/src/core/codegen/bindings.ts"), &types)
        .inspect_err(|e| {
            println!("Error: {:?}", e);
        }).expect("Failed to compile rpc types");
    }
}

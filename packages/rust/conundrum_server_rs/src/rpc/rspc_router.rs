use std::ops::Index;

use conundrum_db::vector::{
    database::schema_version::{schema_version::SchemaVersion, server_version::ServerVersion},
    models::ecosystem_data::ecosystem_data::VersionData,
};
use rspc::Procedure;

use crate::{
    errors::server_error::{ServerError, ServerResult},
    rpc::{
        route_context::RouteContext,
        routers::{
            cdrm::cdrm_router::get_cdrm_router, code::code_router::get_code_router,
            crud::nested_crud_router::get_nested_crud_router, describe::describe_router::get_describe_router,
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
    let crud_router = get_nested_crud_router();

    let r = rspc::Router::<RouteContext>::new().nest("fs", fs_router)
                                               .nest("workspace_management", workspace_router)
                                               .nest("code", code_router)
                                               .nest("cdrm", cdrm_router)
                                               .nest("describe", describe_router)
                                               .nest("crud", crud_router)
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

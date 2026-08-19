use std::{ops::Index, sync::Arc};

use conundrum::ecosystem::{
    db::db_traits::async_traits::try_from_async::FromAsync,
    error_handling::server_error::{ServerError, ServerResult},
};
use conundrum_db::vector::{
    database::schema_version::{schema_version::SchemaVersion, server_version::ServerVersion},
    models::ecosystem_data::{
        backend_status::BackendStatus, ecosystem_data::VersionData, server_state::server_state::ServerState,
    },
};
use rspc::Procedure;

use crate::rpc::{
    routers::{
        cdrm::cdrm_router::get_cdrm_router, code::code_router::get_code_router,
        crud::nested_crud_router::get_nested_crud_router, describe::describe_router::get_describe_router,
        fs::fs_router::get_fs_router, initialization::initialization_router::get_initialization_router,
        log::logger_router::get_logger_router, table::table_router::get_table_router,
        workspace_management::workspace_management_router::get_workspace_management_router,
    },
    server_health::server_health::ServerHealthReport,
};

pub async fn get_rspc_router() -> ServerResult<(rspc::Procedures<Arc<ServerState>>, rspc::Types)> {
    let fs_router = get_fs_router();
    let logger_router = get_logger_router();
    let table_router = get_table_router();
    let initialization_router = get_initialization_router();
    // let mut study_router = get_study_router();
    let workspace_router = get_workspace_management_router();
    let code_router = get_code_router();
    let cdrm_router = get_cdrm_router();
    let describe_router = get_describe_router();
    let crud_router = get_nested_crud_router();

    let r = rspc::Router::<Arc<ServerState>>::new().nest("fs", fs_router)
                                               .nest("workspace_management", workspace_router)
                                               .nest("code", code_router)
                                               .nest("log", logger_router)
                                               .nest("tables", table_router)
                                               .nest("cdrm", cdrm_router)
                                               .nest("describe", describe_router)
                                               .nest("crud", crud_router)
                                               .nest("initialize", initialization_router)
                                               .procedure("backend_status", Procedure::builder::<ServerError>().query(|ctx: Arc<ServerState>, _: ()| async move {
                                                              let status = BackendStatus::from_async(ctx.clone()).await;
                                                              Ok(status)
                                                          }))
                                               .procedure("rpc_health",
                                                          Procedure::builder::<ServerError>().query(|ctx: Arc<ServerState>, _: ()| async move {
                                                              let db = ctx.db.clone();
                                                              let health = ServerHealthReport::new(&db).await?;
                                                              Ok(health)
                                                          }))
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

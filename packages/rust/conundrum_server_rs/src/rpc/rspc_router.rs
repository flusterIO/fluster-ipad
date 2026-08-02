use conundrum_db::vector::{
    database::schema_version::{schema_version::SchemaVersion, server_version::ServerVersion},
    models::ecosystem_data::ecosystem_data::VersionData,
};
use rspc::Procedure;

use crate::{
    errors::server_error::ServerError,
    rpc::{route_context::RouteContext, routers::fs_router::fs_router::get_fs_router},
};

pub fn get_rspc_router() -> (rspc::Procedures<RouteContext>, rspc::Types) {
    let mut fs_router = get_fs_router();
    rspc::Router::<RouteContext>::new()
        .nest("fs", fs_router)
                                       .procedure("version",
                                                  Procedure::builder::<ServerError>().query(|_, _: ()| async {
                                                                                         Ok(VersionData { server:
                                                                                     ServerVersion::current_version(),
                                                                                 database:
                                                                                     SchemaVersion::current_version() })
                                                                                     }))
                                       .build()
                                       .expect("Must always build server without throwing an error.")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rspc::Typescript;

    use super::*;

    #[test]
    fn write_rspc_route_types() {
        let (_, types) = get_rspc_router();
        Typescript::default()
        .enable_source_maps()
        .export_to(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../conundrum_frontend/src/core/codegen/bindings.ts"), &types);
    }
}

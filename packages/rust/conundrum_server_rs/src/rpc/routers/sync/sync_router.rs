use std::sync::Arc;

use crate::rpc::routers::fs::fs_path_simple_result::{FSPathSimpleResult, PathVariant};
use conundrum::ecosystem::error_handling::server_error::{ServerError, ServerResult};
use conundrum::lifted_models::primitives::db_id::DatabaseId;
use conundrum_db::vector::{
    database::helper_crud_functions::save_entity::save_entities,
    models::ecosystem_data::{log::ecosystem_log_input::EcosystemLogInput, server_state::server_state::ServerState},
};
use rspc::{Procedure, Router};

pub fn get_sync_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new()
        .procedure("create",
                                            Procedure::<Arc<ServerState>, EcosystemLogInput, ()>::builder::<ServerError>().mutation(|ctx: Arc<ServerState>, req: EcosystemLogInput| async move {
                                                Ok(())
                                                                               }))
}

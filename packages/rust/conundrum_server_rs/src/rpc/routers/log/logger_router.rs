use std::sync::Arc;

use crate::rpc::routers::fs::fs_path_simple_result::{FSPathSimpleResult, PathVariant};
use conundrum::ecosystem::error_handling::server_error::{ServerError, ServerResult};
use conundrum::{ecosystem::db::db_traits::entity_crud::EntityCRUD, lifted_models::primitives::db_id::DatabaseId};
use conundrum_db::vector::{
    database::helper_crud_functions::save_entity::save_entities,
    models::ecosystem_data::{
        log::{ecosystem_log::EcosystemLog, ecosystem_log_input::EcosystemLogInput},
        server_state::server_state::ServerState,
    },
    parameters::general::general_query::GeneralQuery,
};
use rspc::{Procedure, Router};

pub fn get_logger_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new()
        .procedure("create",
                                            Procedure::<Arc<ServerState>, EcosystemLogInput, ()>::builder::<ServerError>().mutation(|ctx: Arc<ServerState>, req: EcosystemLogInput| async move {
                                                let item: EcosystemLog = req.into();
                                                let db = ctx.db.clone();
                                                save_entities::<'_, EcosystemLog, DatabaseId>(vec![
                                                    item
                                                ], db).await.map_err(ServerError::DatabaseError)?;
                                                Ok(())
                                                                               }))
        .procedure("get_many",
                                            Procedure::<Arc<ServerState>, GeneralQuery, Vec<EcosystemLog>>::builder::<ServerError>().query(|ctx: Arc<ServerState>, req: GeneralQuery| async move {
                                                let db = ctx.db.clone();
                                                let items = EcosystemLog::get_by_predicate(req.predicate, Some(req.pagination), req.sort, &db).await.map_err(ServerError::DatabaseError)?;
                                                Ok(items)
                                                                               }))
}

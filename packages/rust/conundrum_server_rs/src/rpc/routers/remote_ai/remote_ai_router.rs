use std::sync::Arc;

use crate::rpc::routers::fs::fs_path_simple_result::{FSPathSimpleResult, PathVariant};
use conundrum::ai::models::model::model_description::ModelDescription;
use conundrum::ai::rig::ai_traits::ai_client_container::AIClientContainer;
use conundrum::ecosystem::error_handling::ai_error::AIError;
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

pub fn get_remote_ai_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new()
        .procedure("list_models",
                                            Procedure::<Arc<ServerState>, (), Vec<ModelDescription>>::builder::<ServerError>().query(|ctx: Arc<ServerState>, req: ()| async move {
                                                let agent = ctx.remote_client.as_ref().cloned().ok_or_else(|| {
                                                        AIError::InvalidRemoteProvider
                                                })?.clone().lock_owned().await;
                                                agent.list_models().await
                                                    .map_err(|e| {
                                                        ServerError::AIError(e)
                                                    })
                                                                               }))
}

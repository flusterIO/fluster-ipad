use std::sync::Arc;

use crate::mcp::create_tool_index::create_tool_index;
use conundrum::ecosystem::{
    db::db_traits::async_traits::try_from_async::FromAsync, error_handling::server_error::ServerError,
};
use conundrum_db::vector::{
    database::inititialize_db::initialize_db::initialize_local_database,
    models::ecosystem_data::{backend_status::BackendStatus, server_state::server_state::ServerState},
};
use rspc::{Procedure, Router};
use serde::{Deserialize, Serialize};

/// Specta keeeps serialing null as an empty map, so we're doing this
/// apparently...
#[derive(Serialize, Deserialize, specta::Type)]
pub struct EmptyStructBecauseSpectaFuckingSucks {}

pub fn get_initialization_router() -> Router<Arc<ServerState>> {
    Router::<Arc<ServerState>>::new()
        .procedure("step_1_init_db",
                                            Procedure::<Arc<ServerState>, EmptyStructBecauseSpectaFuckingSucks, BackendStatus>::builder::<ServerError>().mutation(|state: Arc<ServerState>, _: EmptyStructBecauseSpectaFuckingSucks| async move {
                                                let cloned_state = Arc::clone(&state);
                                                initialize_local_database(&cloned_state).await?;
                                                let health = BackendStatus::from_async(Arc::clone(&state)).await;
                                                                                   Ok(health)
                                                                               }))
        .procedure("step_2_init_tool_index",
                                            Procedure::<Arc<ServerState>, EmptyStructBecauseSpectaFuckingSucks, BackendStatus>::builder::<ServerError>().mutation(|state: Arc<ServerState>, _: EmptyStructBecauseSpectaFuckingSucks| async move {
                                                let cloned_state = Arc::clone(&state);
                                                let db = &cloned_state.db;
                                                create_tool_index(db).await?;
                                                let health = BackendStatus::from_async(Arc::clone(&state)).await;
                                                                                   Ok(health)
                                                                               }))
}

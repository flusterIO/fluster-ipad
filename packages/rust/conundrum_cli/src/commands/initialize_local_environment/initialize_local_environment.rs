use std::sync::Arc;

use conundrum::ai::rig::rig_client_remote::RigClientRemote;
use conundrum_db::vector::{
    database::inititialize_db::initialize_db::initialize_local_database,
    models::ecosystem_data::server_state::server_state::ServerState,
};

use crate::errors::{ConundrumCliError, ConundrumCliResult};

pub async fn initialize_local_environment() -> ConundrumCliResult<()> {
    let client = RigClientRemote::initialize().map_err(ConundrumCliError::AIError)?;
    let arc_mutex_client = Arc::new(tokio::sync::Mutex::new(client));
    let handler = cdrm_server_lib::mcp::mcp_handler::ConundrumMCP::default();
    let server_state = ServerState::try_new(handler).await.map_err(|e| {
                                                               log::error!("Error: {:#?}", e);
                                                               ConundrumCliError::DatabaseError(e)
                                                           })?;
    let arc_state = Arc::new(server_state);
    initialize_local_database(&arc_state).await.map_err(|e| {
                                                    log::error!("Failed to initialize with the following error: {:#?}",
                                                                e);
                                                    ConundrumCliError::DatabaseError(e)
                                                })?;
    Ok(())
}

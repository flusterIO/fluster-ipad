use std::sync::Arc;

use conundrum::ai::rig::rig_client_remote::RigClientRemote;
use conundrum_db::vector::database::inititialize_db::initialize_db::initialize_local_database;

use crate::errors::{ConundrumCliError, ConundrumCliResult};

pub async fn initialize_local_environment() -> ConundrumCliResult<()> {
    let client = RigClientRemote::initialize().map_err(ConundrumCliError::AIError)?;
    let arc_mutex_client = Arc::new(tokio::sync::Mutex::new(client));
    initialize_local_database(&arc_mutex_client).await.map_err(|e| {
        log::error!("Failed to initialize with the following error: {:#?}", e);
        ConundrumCliError::DatabaseError(e)
    })?;
    Ok(())
}

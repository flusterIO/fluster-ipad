use conundrum_db::vector::database::inititialize_db::initialize_db::initialize_local_database;

use crate::errors::ConundrumCliResult;

pub async fn initialize_local_environment() -> ConundrumCliResult<()> {
    initialize_local_database();
    Ok(())
}

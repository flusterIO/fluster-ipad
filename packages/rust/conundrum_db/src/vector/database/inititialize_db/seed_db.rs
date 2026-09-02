use std::sync::Arc;

use conundrum::{
    ecosystem::{db::db_client::db_client::DBClient, error_handling::db_error::DatabaseResult},
    lang::runtime::run_conundrum::ParseConundrumOptions,
};

use crate::vector::{
    models::ecosystem_data::server_state::server_state::ServerState,
    seed::{
        seed_content::{SeedChunks, SeedContent},
        seed_documentation::seed_documentation::SeedDocumentation,
        seed_settings::settings_seeder::SettingsSeeder,
    },
};

pub async fn seed_db(db: DBClient, state: std::sync::Arc<ServerState>) -> DatabaseResult<()> {
    SeedDocumentation::default().try_seed(db.clone(), ParseConundrumOptions::for_chunking(), Arc::clone(&state))
                                .await?;
    SettingsSeeder::default().try_seed(db.clone()).await?;
    Ok(())
}

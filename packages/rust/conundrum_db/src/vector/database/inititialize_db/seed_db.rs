use std::sync::Arc;

use conundrum::{
    ecosystem::{db::db::ArcMutexDB, error_handling::db_error::DatabaseResult},
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

pub async fn seed_db(db: ArcMutexDB, client: &std::sync::Arc<ServerState>) -> DatabaseResult<()> {
    SeedDocumentation::default().try_seed(db, ParseConundrumOptions::default(), client).await?;
    SettingsSeeder {}.try_seed(Arc::clone(&db)).await?;
    Ok(())
}

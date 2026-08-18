use conundrum::{
    ecosystem::{db::db::ArcMutexDB, error_handling::db_error::DatabaseResult},
    lang::runtime::run_conundrum::ParseConundrumOptions,
};

use crate::vector::{
    models::ecosystem_data::server_state::server_state::ServerState,
    seed::{seed_content::SeedContent, seed_documentation::seed_documentation::SeedDocumentation},
};

pub async fn seed_db(db: &ArcMutexDB, client: &std::sync::Arc<ServerState>) -> DatabaseResult<()> {
    SeedDocumentation::default().try_seed(db,
                                          ParseConundrumOptions { note_id:
                                                                      Some("Conundrum Documentation".to_string()),
                                                                  ..Default::default() },
                                          client)
                                .await?;
    Ok(())
}

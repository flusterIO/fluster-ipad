use std::sync::Arc;

use cdrm_server_lib::mcp::mcp_handler::ConundrumMCP;
use conundrum_db::vector::{
    database::inititialize_db::initialize_db::initialize_local_database,
    models::ecosystem_data::server_state::server_state::ServerState,
};

#[test_log::test(tokio::test)]
async fn initializes_database() {
    let server_state = ServerState::try_new().await.expect("Gets server state.");
    let arc_state = Arc::new(server_state);
    initialize_local_database(&Arc::clone(&arc_state)).await
                                                      .inspect_err(|e| {
                                                          log::error!("Error: {:?}", e);
                                                      })
                                                      .expect("Initializes database.");
}

use std::sync::Arc;

use cdrm_server_lib::mcp::mcp_handler::ConundrumMCP;
use conundrum_db::vector::{
    database::inititialize_db::initialize_db::initialize_local_database,
    models::ecosystem_data::server_state::server_state::ServerState,
};

#[test_log::test(tokio::test)]
async fn initializes_database() {
    let handler = ConundrumMCP {};
    // let arc_handler = Arc::new(tokio::sync::Mutex::new(handler));
    let server_state = ServerState::try_new(handler).await.expect("Gets server state.");
    let client = server_state.local_client.expect("Must have local client.");
    initialize_local_database(&client).await
                                      .inspect_err(|e| {
                                          log::error!("Error: {:?}", e);
                                      })
                                      .expect("Initializes database.")
}

use std::sync::Arc;

use conundrum_lang::ecosystem::db::{db::get_database, db_client::db_client::DBClient};
use lancedb::Connection;
use tokio::sync::Mutex;

pub async fn get_test_database() -> DBClient {
    get_database().await.expect("Must get database.")
}

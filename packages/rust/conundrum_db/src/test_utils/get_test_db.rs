use std::sync::Arc;

use conundrum::ecosystem::db::db::get_database;
use lancedb::Connection;
use tokio::sync::Mutex;

pub async fn get_test_database() -> Arc<Mutex<Connection>> {
    get_database().await.expect("Must get database.")
}

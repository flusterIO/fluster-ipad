use std::sync::Arc;

use lancedb::Connection;
use tokio::sync::Mutex;

use crate::vector::database::db::get_database;

pub async fn get_test_database() -> Arc<Mutex<Connection>> {
    get_database().await.expect("Must get database.")
}

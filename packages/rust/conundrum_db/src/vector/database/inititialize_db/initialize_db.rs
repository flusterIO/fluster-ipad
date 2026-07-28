use askama::Template;
use conundrum::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};

use crate::vector::database::{db::get_database, global_queries::initialize_database_query::InitializeDatabaseQuery};

pub async fn initialize_local_database() -> DatabaseResult<()> {
    let query = InitializeDatabaseQuery::default();
    let schema = query.render().map_err(|_| DatabaseError::FailToSerialize("Conundrum Schema error".to_string()))?;
    println!("Schema: {}", schema);
    let db = get_database().await?;
    let locked_db = db.lock().await;
    locked_db.use_ns("main").use_db("conundrum").await.map_err(|_| {
                                                          log::error!("Failed to connect to the conundrum database while generating initial data.");
                                                          DatabaseError::FailToConnect
                                                      })?;
    locked_db.query(schema).await.map_err(|e| DatabaseError::DatabaseError { source: Some(e) })?;
    drop(locked_db);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn initializes_local_database() {
        initialize_local_database().await.expect("Initializes local database without throwing an error.");
    }
}

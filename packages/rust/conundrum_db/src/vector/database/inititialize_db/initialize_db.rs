use askama::Template;
use conundrum::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};

use crate::vector::database::{db::get_database, global_queries::initialize_database_query::InitializeDatabaseQuery};

pub async fn initialize_local_database() -> DatabaseResult<()> {
    let query = InitializeDatabaseQuery::default();
    let schema = query.render().map_err(|_| DatabaseError::FailToSerialize("Conundrum Schema error".to_string()))?;
    println!("Schema: {}", schema);
    let db = get_database().await?;
    let locked_db = db.clone().lock_owned().await;
    locked_db.query(schema).await.map_err(|e| DatabaseError::DatabaseError { source: Some(e) })?;
    drop(locked_db);
    Ok(())
}

#[cfg(test)]
mod tests {

    use conundrum_fs::path_utils::ecosystem_paths::get_app_database_dir;

    use super::*;

    #[test]
    fn print_database_path() {
        println!("{:?}", get_app_database_dir().expect("Mother-er you better print"));
    }

    #[tokio::test]
    async fn initializes_local_database() {
        initialize_local_database().await.expect("Initializes local database without throwing an error.");
    }
}

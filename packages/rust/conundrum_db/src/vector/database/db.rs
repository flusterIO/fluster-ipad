use conundrum::ecosystem::db::tables::DatabaseTable;
use conundrum::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};
use lancedb::Table;
use lancedb::{Connection, connect};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::OnceCell;
use tokio::sync::{Mutex, MutexGuard};

pub type CdrmDb = Connection;

pub type CdrmDbGuard<'a> = MutexGuard<'a, CdrmDb>;

static DB: OnceCell<DatabaseResult<Arc<Mutex<CdrmDb>>>> = OnceCell::const_new();

pub fn get_data_dir() -> DatabaseResult<PathBuf> {
    let mut d = dirs::data_local_dir();
    if d.is_none() {
        d = dirs::data_local_dir();
    }
    if d.is_none() {
        return Err(DatabaseError::FailToFindDataDirectory);
    }
    Ok(d.unwrap().join("conundrum").join("data"))
}

pub async fn get_table(conn: &CdrmDbGuard<'_>, tbl: DatabaseTable) -> DatabaseResult<Table> {
    conn.open_table(tbl.to_string()).execute().await.map_err(|e| {
                                                        println!("Error in get_table: {:?}", e);
                                                        DatabaseError::FailToConnect
                                                    })
}

pub fn get_database_path() -> DatabaseResult<PathBuf> {
    let d = get_data_dir()?;
    Ok(d.join("database"))
}

pub async fn get_database() -> DatabaseResult<Arc<Mutex<Connection>>> {
    DB.get_or_init(|| async {
          let db_path = get_database_path()?;
          let db = connect(db_path.to_str().unwrap()).execute().await.expect("Failed to connect to database.");
          Ok(Arc::new(Mutex::new(db)))
      })
      .await
      .clone()
}

// pub async fn clean_table(db: &CdrmDbGuard<'_>, tb: DatabaseTable) ->
// FlusterResult<()> {     let tbl = get_table(db, tb).await?;
//     // Pass in a predicate that always evaluates to true to delete all items.
//     tbl.delete("1 = 1").await.map_err(|e| {
//                                   println!("Error in clean_table: {:?}", e);
//                                   DatabaseError::FailToDelete(tb)
//                               })?;
//     Ok(())
// }

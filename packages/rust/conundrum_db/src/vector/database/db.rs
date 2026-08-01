use conundrum::ecosystem::db::tables::DatabaseTable;
use conundrum::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};
use conundrum_fs::path_utils::ecosystem_paths::get_app_database_dir;
use lancedb::Table;
use lancedb::{Connection, connect};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};
use tokio::sync::{OnceCell, OwnedMutexGuard};

pub type CdrmDb = Connection;

pub type DBGuard = OwnedMutexGuard<CdrmDb>;

pub type ArcMutexDB = Arc<Mutex<CdrmDb>>;

static DB: OnceCell<DatabaseResult<ArcMutexDB>> = OnceCell::const_new();

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

pub async fn get_database() -> DatabaseResult<ArcMutexDB> {
    DB.get_or_init(|| async {
          let db_path = get_app_database_dir().map_err(DatabaseError::FileSystemError)?;
          let db = connect(db_path.to_str().unwrap()).execute().await.map_err(|e| {
                                                                          log::error!("Error: {:?}", e);
                                                                          DatabaseError::FailToConnect
                                                                      })?;
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

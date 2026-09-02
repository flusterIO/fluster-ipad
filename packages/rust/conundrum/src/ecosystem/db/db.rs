use crate::ecosystem::db::db_client::db_client::DBClient;
use crate::ecosystem::ecosystem_paths::get_app_database_dir;
use crate::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};
use lancedb::{Connection, connect};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::{OnceCell, OwnedMutexGuard};

pub type CdrmDb = Connection;

pub type DBGuard = OwnedMutexGuard<CdrmDb>;

static DB: OnceCell<DatabaseResult<DBClient>> = OnceCell::const_new();

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

pub async fn get_database() -> DatabaseResult<DBClient> {
    DB.get_or_init(|| async {
          let db_path = get_app_database_dir().map_err(DatabaseError::FileSystemError)?;
          let db = connect(db_path.to_str().unwrap()).execute().await.map_err(|e| {
                                                                          log::error!("Error: {:?}", e);
                                                                          DatabaseError::FailToConnect
                                                                      })?;
          Ok(DBClient(Arc::new(Mutex::new(db))))
      })
      .await
      .clone()
}

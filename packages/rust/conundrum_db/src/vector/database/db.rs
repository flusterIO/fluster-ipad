use conundrum::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};
use std::{path::PathBuf, sync::Arc};
use surrealdb::{
    Surreal,
    engine::local::{Db, RocksDb},
};
use tokio::sync::{Mutex, OnceCell};

pub type CDRMSurrealDB = Surreal<Db>;
pub type ArcMutexDB = Arc<Mutex<CDRMSurrealDB>>;

pub fn get_app_data_dir() -> DatabaseResult<PathBuf> {
    if let Some(d) = dirs::data_local_dir() {
        Ok(d.join("conundrum"))
    } else {
        dirs::data_dir().map(|x| x.join("conundrum")).ok_or(DatabaseError::InvalidDataDirectory)
    }
}

/// Deprecated... use the one from the fs crate.
pub fn get_app_database_dir() -> DatabaseResult<PathBuf> {
    Ok(get_app_data_dir()?.join("database"))
}

static DB: OnceCell<ArcMutexDB> = OnceCell::const_new();

pub async fn get_database<'a>() -> DatabaseResult<&'a ArcMutexDB> {
    DB.get_or_try_init::<DatabaseError, _, _>(|| async {
          let database_dir = get_app_database_dir()?;
          log::debug!("Database Directory: {:#?}", database_dir);
          let c = Surreal::new::<RocksDb>(database_dir).await.map_err(|e| {
                                                                  log::error!("Database Connection Error: {}", e);
                                                                  DatabaseError::FailToConnect
                                                              })?;
          c.use_ns("conundrum").use_db("main").await.map_err(|e| {
                                                         log::error!("Error: {:?}", e);
                                                         DatabaseError::FailToConnect
                                                     })?;
          Ok(Arc::new(Mutex::new(c)))
      })
      .await
}

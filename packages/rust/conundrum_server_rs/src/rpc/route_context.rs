use conundrum_db::vector::database::db::{ArcMutexDB, CdrmDb, get_database};

use crate::errors::server_error::{ServerError, ServerResult};

#[derive(Clone)]
pub struct RouteContext {
    pub db: ArcMutexDB,
}

impl RouteContext {
    pub async fn try_new() -> ServerResult<Self> {
        let db = get_database().await.map_err(|e| ServerError::DatabaseError(e))?;
        Ok(Self { db })
    }
}

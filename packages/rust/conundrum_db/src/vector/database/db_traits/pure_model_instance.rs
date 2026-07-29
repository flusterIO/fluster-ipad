use async_trait::async_trait;
use conundrum::ecosystem::error_handling::db_error::DatabaseResult;
use surrealdb_types::RecordId;

use crate::vector::{database::db::ArcMutexDB, models::primitives::id_record::IDRecord};

pub trait PureModelInstanceMethods: Send + Sync {
    async fn upsert_self(&self, db: &ArcMutexDB) -> DatabaseResult<RecordId>;
}

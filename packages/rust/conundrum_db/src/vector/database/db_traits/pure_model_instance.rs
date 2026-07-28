use async_trait::async_trait;
use conundrum::ecosystem::error_handling::db_error::DatabaseResult;

use crate::vector::{database::db::ArcMutexDB, models::primitives::id_record::IDRecord};

pub trait PureModelInstanceMethods: Send + Sync {
    async fn save_self(&self, db: &ArcMutexDB) -> DatabaseResult<IDRecord>;
}

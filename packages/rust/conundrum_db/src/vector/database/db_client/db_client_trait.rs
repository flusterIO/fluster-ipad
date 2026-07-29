use conundrum::ecosystem::{
    db::tables::DatabaseTable,
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use surrealdb_types::SurrealValue;

use crate::vector::database::db::ArcMutexDB;

pub trait CDRMDatabaseClient {
    async fn get_all_from_table<T: SurrealValue>(db: &ArcMutexDB, tbl: &DatabaseTable) -> DatabaseResult<Vec<T>>;
}

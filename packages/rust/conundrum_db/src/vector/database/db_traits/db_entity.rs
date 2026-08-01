use std::sync::Arc;

use arrow_array::RecordBatch;
use conundrum::ecosystem::{db::tables::DatabaseTable, error_handling::db_error::DatabaseResult};
use lancedb::arrow::arrow_schema::Schema;

use crate::vector::database::db::DBGuard;

/// An entity refers to an object *exactly* as it appears in the database, or as
/// close to that as we can get with Rust types. If you're looking for something
/// more composed and usable, look into the equivalent 'Model'.
pub trait DBEntity {
    fn arrow_schema() -> Arc<Schema>;
    fn table() -> DatabaseTable;
    fn get_record_batch(data: Vec<Self>) -> DatabaseResult<RecordBatch>
        where Self: Sized {
        todo!()
    }
    fn merge_keys() -> &'static [&'static str] {
        todo!()
    }
    fn primary_key() -> &'static str {
        todo!()
    }
    // fn save_self(&self, db: &ArcMutexDB) -> DatabaseResult<()>;
}

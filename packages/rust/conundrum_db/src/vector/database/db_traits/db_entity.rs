use std::sync::Arc;

use arrow_array::RecordBatch;
use conundrum::ecosystem::{
    db::tables::DatabaseTable,
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use lancedb::arrow::arrow_schema::Schema;
use serde::Serialize;
use serde_arrow::to_record_batch;

pub trait ArrowSchemaRepresentable {
    fn arrow_schema() -> Arc<Schema>;
}

/// An entity refers to an object *exactly* as it appears in the database, or as
/// close to that as we can get with Rust types. If you're looking for something
/// more composed and usable, look into the equivalent 'Model'.
pub trait DBEntity<PrimaryValueType = String>: ArrowSchemaRepresentable {
    type PartialUpdateType;
    fn table() -> DatabaseTable;
    fn get_record_batch(data: Vec<Self>) -> DatabaseResult<RecordBatch>
        where Self: Sized + Clone + Serialize {
        to_record_batch(&Self::arrow_schema().fields, &data.clone()).map_err(|e| {
                                                                        log::error!("Error: {:?}", e);
                                                                        DatabaseError::SerializationError
                                                                    })
    }
    fn merge_keys() -> &'static [&'static str];
    fn primary_key() -> &'static str;
    fn primary_value(&self) -> PrimaryValueType;
    // fn save_self(&self, db: &ArcMutexDB) -> DatabaseResult<()>;
}

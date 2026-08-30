use std::sync::Arc;

use crate::ecosystem::{
    db::{
        db_traits::db_identifiable::DatabaseIdentifiable, parameters::ai::schema_parameters::SchemaParameters,
        tables::DatabaseTable,
    },
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use arrow_array::RecordBatch;
use arrow_schema::{Field, FieldRef};
use fake::{Dummy, Faker};
use lancedb::arrow::arrow_schema::Schema;
use serde::{Deserialize, Serialize};
use serde_arrow::{
    schema::{SchemaLike, TracingOptions},
    to_record_batch,
};

/// Deprecated, I'm pretty sure.
pub trait DBSchema: ArrowFields {
    fn schema() -> DatabaseResult<Schema> {
        let fields = Self::arrow_fields()?;
        let schema = arrow_schema::Schema::new(fields);
        Ok(schema)
    }
}

pub trait ArrowFields {
    fn arrow_fields() -> DatabaseResult<Vec<Arc<Field>>>;
}

/// An entity refers to an object *exactly* as it appears in the database, or as
/// close to that as we can get with Rust types. If you're looking for something
/// more composed and usable, look into the equivalent 'Model'.
pub trait DBEntity: DBSchema
    where <Self as DBEntity>::IDType: DatabaseIdentifiable {
    type PartialUpdateType;
    type IDType;
    fn table() -> DatabaseTable;
    fn get_record_batch(data: Vec<Self>) -> DatabaseResult<RecordBatch>
        where Self: Sized + Clone + Serialize {
        let fields = Self::arrow_fields()?;
        to_record_batch(&fields, &data.clone()).map_err(|e| {
                                                   log::error!("Error: {:?}", e);
                                                   DatabaseError::SerializationError
                                               })
    }
    fn merge_keys() -> &'static [&'static str];
    fn primary_key() -> &'static str;
    fn primary_value(&self) -> <Self as DBEntity>::IDType;
    fn set_primary_value(&mut self, value: <Self as DBEntity>::IDType);
    // fn save_self(&self, db: &ArcMutexDB) -> DatabaseResult<()>;
}

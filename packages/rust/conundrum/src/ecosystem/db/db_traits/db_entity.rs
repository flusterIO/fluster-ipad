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
    type IDType: DatabaseIdentifiable;
    fn schema() -> DatabaseResult<Schema> {
        let fields = Self::arrow_fields()?;
        let schema = arrow_schema::Schema::new(fields);
        Ok(schema)
    }
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
    fn primary_value(&self) -> <Self as DBSchema>::IDType;
    fn set_primary_value(&mut self, value: <Self as DBSchema>::IDType);
}

pub trait ArrowFields {
    fn arrow_fields() -> DatabaseResult<Vec<Arc<Field>>>;
}

/// An entity refers to an object *exactly* as it appears in the database, or as
/// close to that as we can get with Rust types. If you're looking for something
/// more composed and usable, look into the equivalent 'Model'.
pub trait DBEntity: DBSchema
    where <Self as DBSchema>::IDType: DatabaseIdentifiable {
    type PartialUpdateType;
    fn table() -> DatabaseTable;
    // fn save_self(&self, db: &ArcMutexDB) -> DatabaseResult<()>;
}

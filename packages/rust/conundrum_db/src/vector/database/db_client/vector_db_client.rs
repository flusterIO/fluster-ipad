use arrow_array::{RecordBatch, RecordBatchIterator};
use conundrum::ecosystem::{
    db::tables::DatabaseTable,
    error_handling::db_error::{DatabaseError, DatabaseResult},
};

use crate::vector::{database::db_types::db_entity::DBEntity, models::taggables::tag::Tag};

pub struct CDRMVectorDB {}

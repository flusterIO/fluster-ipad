use std::sync::Arc;

use arrow_array::{RecordBatch, StringArray};
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_types::db_entity::DBEntity,
    models::{
        date_time::date_time::DateTime, primitives::case_insensitive_string::CaseInsensitiveString,
        taggables::tag_location::TagLocation,
    },
};

use conundrum::ecosystem::{
    db::traits::database_field_representable::DatabaseFieldRepresentable,
    error_handling::db_error::{DatabaseError, DatabaseResult},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tag {
    pub body: CaseInsensitiveString,
    pub location: TagLocation,
    pub ctime: DateTime,
}

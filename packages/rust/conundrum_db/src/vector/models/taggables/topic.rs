use std::sync::Arc;

use fake::Dummy;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
    get_taggable_recordbatch, taggable_arrow_schema,
    vector::{
        database::db_traits::{db_entity::DBEntity, db_field::DatabaseField},
        models::{
            date_time::date_time::DateTime,
            primitives::case_insensitive_string::CaseInsensitiveString,
            taggables::{
                tag::{TAGGABLE_MERGE_KEYS, TAGGABLE_PRIMARY_KEY},
                tag_location::TagLocation,
            },
        },
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy, Type)]
pub struct Topic {
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ctime: DateTime,
    pub last_access: DateTime,
}

impl From<String> for Topic {
    fn from(value: String) -> Self {
        Topic { value: value.into(),
                location: TagLocation::Straggling,
                ctime: DateTime::new_now(),
                last_access: DateTime::new_now() }
    }
}

impl DBEntity for Topic {
    fn arrow_schema() -> std::sync::Arc<lancedb::arrow::arrow_schema::Schema> {
        taggable_arrow_schema!()
    }

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::Topic
    }

    fn get_record_batch(data: Vec<Self>)
                        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<arrow_array::RecordBatch>
        where Self: Sized {
        get_taggable_recordbatch!(data)
    }

    fn merge_keys() -> &'static [&'static str] {
        TAGGABLE_MERGE_KEYS
    }

    fn primary_key() -> &'static str {
        TAGGABLE_PRIMARY_KEY
    }
}

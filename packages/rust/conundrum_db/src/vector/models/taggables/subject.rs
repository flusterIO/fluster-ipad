use std::sync::Arc;

use arrow_array::{Date64Array, RecordBatch, StringArray};
use conundrum::ecosystem::error_handling::db_error::DatabaseResult;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::{
    get_taggable_recordbatch, taggable_arrow_schema,
    vector::{
        database::db_traits::{
            db_entity::{ArrowSchemaRepresentable, DBEntity},
            db_field::DatabaseField,
        },
        models::{
            date_time::date_time::DateTime,
            primitives::{case_insensitive_string::CaseInsensitiveString, db_id::DatabaseId},
            taggables::{
                tag::{TAGGABLE_MERGE_KEYS, TAGGABLE_PRIMARY_KEY},
                tag_location::TagLocation,
                taggable_update_partial::TaggablePartial,
            },
        },
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy)]
pub struct Subject {
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ctime: DateTime,
    pub last_access: DateTime,
}

impl From<String> for Subject {
    fn from(value: String) -> Self {
        Subject { value: CaseInsensitiveString::from(value),
                  location: TagLocation::Body,
                  ctime: DateTime::new_now(),
                  last_access: DateTime::new_now() }
    }
}

impl ArrowSchemaRepresentable for Subject {
    fn arrow_schema() -> std::sync::Arc<lancedb::arrow::arrow_schema::Schema> {
        taggable_arrow_schema!()
    }
}

impl DBEntity for Subject {
    type PartialUpdateType = TaggablePartial;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::Subject
    }

    fn get_record_batch(data: Vec<Self>) -> DatabaseResult<arrow_array::RecordBatch>
        where Self: Sized {
        get_taggable_recordbatch!(data)
    }

    fn merge_keys() -> &'static [&'static str] {
        TAGGABLE_MERGE_KEYS
    }

    fn primary_key() -> &'static str {
        TAGGABLE_PRIMARY_KEY
    }

    fn primary_value(&self) -> String {
        self.value.to_comparison_string()
    }
}

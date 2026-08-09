use std::sync::Arc;

use arrow_array::{Date64Array, RecordBatch, StringArray};
use conundrum::ecosystem::{
    db::traits::db_entity::{DBEntity, DBSchema},
    error_handling::db_error::DatabaseResult,
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::{
    get_taggable_recordbatch, taggable_arrow_schema,
    vector::{
        database::db_traits::db_field::DatabaseField,
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

/// # Subject
/// A `Subject` is very much like a `Topic`, but in most use cases a `Topic`
/// is more specific than a `Subject`. A user might have a set of subjects like
/// 'physics' and 'math', where they may have a set of topics like
/// 'newtonian-gravity' and 'covariant-derivatives'. However, this is
/// **not** a rule that is set in stone and you should follow whatever pattern
/// the user is using with their tags, topics and subjects.
#[derive(Serialize, Deserialize, Clone, Debug, Dummy, specta::Type)]
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

impl<'a> DBSchema<'a> for Subject {}

impl<'a> DBEntity<'a> for Subject {
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

use std::{default, sync::Arc};

use arrow_array::{RecordBatchIterator, TimestampMillisecondArray};
use conundrum::ecosystem::{
    db::traits::db_entity::{DBEntity, DBSchema},
    error_handling::db_error::{DatabaseError, DatabaseResult},
};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use serde_arrow::schema::SchemaLike;
use specta::Type;

use crate::{
    get_by_predicate, get_taggable_recordbatch, impl_default_crud,
    test_utils::faker_generators::fake_words_as_string::fake_words_as_string,
    vector::{
        database::{
            db_traits::{db_field::DatabaseField, entity_crud::EntityCRUD},
            open_table::open_table,
        },
        models::{
            ai::ai_interactions::AIInteractions,
            date_time::date_time::DateTime,
            primitives::case_insensitive_string::CaseInsensitiveString,
            taggables::{tag_location::TagLocation, taggable_update_partial::TaggablePartial},
        },
    },
};

/// The `_lc` suffix is appended by the `CaseInsensitiveString` struct.
pub static TAGGABLE_PRIMARY_KEY: &str = "value";
pub static TAGGABLE_MERGE_KEYS: &[&str] = &[TAGGABLE_PRIMARY_KEY];

#[derive(Serialize, Deserialize, Clone, Debug, Dummy, Type)]
pub struct Tag {
    #[dummy(faker = "fake_words_as_string(0..10)")]
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ai: AIInteractions,
    #[serde(default = "DateTime::new_now")]
    pub ctime: DateTime,
    #[serde(default = "DateTime::new_now")]
    pub last_access: DateTime,
}

impl From<String> for Tag {
    fn from(value: String) -> Self {
        Tag { value: CaseInsensitiveString::from(value),
              location: TagLocation::Straggling,
              ai: AIInteractions::default(),
              ctime: DateTime::new_now(),
              last_access: DateTime::new_now() }
    }
}

impl<'a> DBSchema<'a> for Tag {
    fn schema_options() -> DatabaseResult<serde_arrow::schema::TracingOptions> {
        serde_arrow::schema::TracingOptions::default().enums_without_data_as_strings(true)
                                                      .overwrite("ctime", DateTime::new_now())
                                                      .map_err(|e| {
                                                          println!("Here?");
                                                          log::error!("Error: {:?}", e);
                                                          DatabaseError::SerializationError
                                                      })?
                                                      .overwrite("last_access", DateTime::new_now())
                                                      .map_err(|e| {
                                                          println!("Here?");
                                                          log::error!("Error: {:?}", e);
                                                          DatabaseError::SerializationError
                                                      })
    }
}

impl_default_crud!(Tag, TaggablePartial, String);

impl<'a> DBEntity<'a> for Tag {
    type PartialUpdateType = TaggablePartial;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::Tag
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

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};

    use crate::test_utils::get_test_db::get_test_database;

    use super::*;

    #[tokio::test]
    async fn saves_tags() {
        let mut test_tags = Vec::new();
        for _ in 0..10 {
            let t: Tag = Faker.fake();
            test_tags.push(t);
        }
        let db = get_test_database().await;
        Tag::save_many(test_tags, &db).await
                                      .inspect_err(|e| {
                                          println!("Error: {:?}", e);
                                      })
                                      .expect("Saves tags");
    }
}

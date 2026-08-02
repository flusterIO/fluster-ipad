use std::sync::Arc;

use arrow_array::{RecordBatchIterator, TimestampMillisecondArray};
use conundrum::ecosystem::error_handling::db_error::{DatabaseError, DatabaseResult};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
    get_taggable_recordbatch, taggable_arrow_schema,
    test_utils::faker_generators::fake_words_as_string::fake_words_as_string,
    vector::{
        database::{
            db_traits::{db_entity::DBEntity, db_field::DatabaseField, entity_crud::EntityCRUD},
            open_table::open_table,
        },
        models::{
            date_time::date_time::DateTime, primitives::case_insensitive_string::CaseInsensitiveString,
            taggables::tag_location::TagLocation,
        },
    },
};

/// The `_lc` suffix is appended by the `CaseInsensitiveString` struct.
pub static TAGGABLE_PRIMARY_KEY: &str = "value_lc";
pub static TAGGABLE_MERGE_KEYS: &[&str] = &[TAGGABLE_PRIMARY_KEY];

#[derive(Serialize, Deserialize, Clone, Debug, Dummy, Type)]
pub struct Tag {
    #[dummy(faker = "fake_words_as_string(0..10)")]
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ctime: DateTime,
    pub last_access: DateTime,
}

impl From<String> for Tag {
    fn from(value: String) -> Self {
        Tag { value: CaseInsensitiveString::from(value),
              location: TagLocation::Straggling,
              ctime: DateTime::new_now(),
              last_access: DateTime::new_now() }
    }
}

impl DBEntity for Tag {
    fn arrow_schema() -> std::sync::Arc<lancedb::arrow::arrow_schema::Schema> {
        taggable_arrow_schema!()
    }

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::Tag
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
}

impl EntityCRUD<String> for Tag {
    async fn delete_by_primary_key(value: String,
                                   table: conundrum::ecosystem::db::tables::DatabaseTable,
                                   db: &crate::vector::database::db::ArcMutexDB)
                                   -> DatabaseResult<()> {
        todo!()
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

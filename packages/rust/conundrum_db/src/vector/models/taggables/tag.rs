use std::sync::Arc;

use arrow_schema::Field;
use conundrum::{
    ecosystem::{
        db::{
            db_traits::{
                db_entity::{DBEntity, DBSchema},
                db_field::DatabaseField,
            },
            tables::DatabaseTable,
        },
        error_handling::db_error::DatabaseResult,
    },
    impl_default_crud,
    lifted_models::primitives::{case_insensitive_string::CaseInsensitiveString, date_time::DateTime},
    testing::faker_generators::fake_words_as_string::fake_words_as_string,
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use specta::Type;

use crate::vector::models::{
    ai::ai_interactions::AIInteractions,
    taggables::{tag_location::TagLocation, taggable_update_partial::TaggablePartial},
};

/// The `_lc` suffix is appended by the `CaseInsensitiveString` struct.
pub static TAGGABLE_PRIMARY_KEY: &str = "value";
pub static TAGGABLE_MERGE_KEYS: &[&str] = &[TAGGABLE_PRIMARY_KEY];

#[derive(Serialize, Deserialize, Clone, Debug, Dummy, Type, DatabaseEntity)]
#[db(table = DatabaseTable::Tag)]
pub struct Tag {
    #[dummy(faker = "fake_words_as_string(0..10)")]
    #[db(primary)]
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ai: AIInteractions,
    pub ctime: DateTime,
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

#[cfg(test)]
mod tests {
    use conundrum::ecosystem::db::db_traits::entity_crud::EntityCRUD;
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

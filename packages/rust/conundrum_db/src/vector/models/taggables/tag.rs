use fake::Dummy;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
    test_utils::faker_generators::fake_words_as_string::fake_words_as_string,
    vector::models::{
        date_time::date_time::DateTime,
        primitives::{case_insensitive_string::CaseInsensitiveString, db_id::DatabaseId},
        taggables::tag_location::TagLocation,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy, Type)]
pub struct Tag {
    #[dummy(faker = "fake_words_as_string(0..10)")]
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ctime: DateTime,
}

impl From<String> for Tag {
    fn from(value: String) -> Self {
        Tag { value: CaseInsensitiveString::from(value),
              location: TagLocation::Straggling,
              ctime: DateTime::new_now() }
    }
}

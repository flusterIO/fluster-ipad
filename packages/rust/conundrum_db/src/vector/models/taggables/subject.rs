use conundrum::ecosystem::db::traits::db_entity::{DBEntity, DBSchema};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::{
    impl_default_crud,
    vector::models::{
        ai::ai_interactions::AIInteractions,
        date_time::date_time::DateTime,
        primitives::{case_insensitive_string::CaseInsensitiveString, db_id::DatabaseId},
        taggables::{
            tag::{TAGGABLE_MERGE_KEYS, TAGGABLE_PRIMARY_KEY, taggable_fields},
            tag_location::TagLocation,
            taggable_update_partial::TaggablePartial,
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
    pub ai: AIInteractions,
}

impl_default_crud!(Subject, TaggablePartial, String);
impl<'a> DBSchema<'a> for Subject {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(taggable_fields())
    }
}

impl From<String> for Subject {
    fn from(value: String) -> Self {
        Subject { value: CaseInsensitiveString::from(value),
                  location: TagLocation::Body,
                  ctime: DateTime::new_now(),
                  last_access: DateTime::new_now(),
                  ai: AIInteractions::default() }
    }
}

impl<'a> DBEntity<'a> for Subject {
    type PartialUpdateType = TaggablePartial;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::Subject
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

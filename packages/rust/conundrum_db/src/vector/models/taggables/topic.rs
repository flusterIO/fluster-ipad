use conundrum::{
    ecosystem::db::db_traits::db_entity::{DBEntity, DBSchema},
    impl_default_crud,
    lifted_models::primitives::{case_insensitive_string::CaseInsensitiveString, date_time::DateTime},
};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::vector::models::{
    ai::ai_interactions::AIInteractions,
    taggables::{
        tag::{TAGGABLE_MERGE_KEYS, TAGGABLE_PRIMARY_KEY, taggable_fields},
        tag_location::TagLocation,
        taggable_update_partial::TaggablePartial,
    },
};

/// # Topic
/// A `Topic` is very much like a `Subject`, but in most use cases a `Subject`
/// is more broad than a `Topic`. A user might have a set of subjects like
/// 'physics' and 'math', where they may have a set of topics like
/// 'newtonian-gravity' and 'covariant-derivatives'. However, this is
/// **not** a rule that is set in stone and you should follow whatever pattern
/// the user is using with their tags, topics and subjects.
#[derive(Serialize, Deserialize, Clone, Debug, Dummy, Type)]
pub struct Topic {
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ctime: DateTime,
    pub last_access: DateTime,
    pub ai: AIInteractions,
}

impl From<String> for Topic {
    fn from(value: String) -> Self {
        Topic { value: value.into(),
                location: TagLocation::Straggling,
                ctime: DateTime::new_now(),
                last_access: DateTime::new_now(),
                ai: AIInteractions::default() }
    }
}

impl<'a> DBSchema<'a> for Topic {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(taggable_fields())
    }
}

impl_default_crud!(Topic, TaggablePartial, String);

impl<'a> DBEntity<'a> for Topic {
    type PartialUpdateType = TaggablePartial;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        conundrum::ecosystem::db::tables::DatabaseTable::Topic
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

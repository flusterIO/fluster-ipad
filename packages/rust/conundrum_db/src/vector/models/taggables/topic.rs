use std::sync::Arc;

use conundrum::ecosystem::db::traits::db_entity::{DBEntity, DBSchema};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
    get_taggable_recordbatch, impl_default_crud, taggable_arrow_schema,
    vector::{
        database::db_traits::{db_field::DatabaseField, entity_crud::EntityCRUD},
        models::{
            ai::ai_interactions::AIInteractions,
            date_time::date_time::DateTime,
            primitives::case_insensitive_string::CaseInsensitiveString,
            taggables::{
                tag::{TAGGABLE_MERGE_KEYS, TAGGABLE_PRIMARY_KEY},
                tag_location::TagLocation,
                taggable_update_partial::TaggablePartial,
            },
            workspace::{user_workspace::UserWorkspace, user_workspace_partial::UserWorkspacePartial},
        },
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

impl<'a> DBSchema<'a> for Topic {}

impl_default_crud!(Topic, TaggablePartial, String);

impl<'a> DBEntity<'a> for Topic {
    type PartialUpdateType = TaggablePartial;

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

    fn primary_value(&self) -> String {
        self.value.to_comparison_string()
    }
}

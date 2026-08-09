use conundrum::ecosystem::db::{
    tables::DatabaseTable,
    traits::db_entity::{DBEntity, DBSchema},
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::{
    impl_default_crud,
    vector::models::{
        date_time::date_time::DateTime,
        primitives::db_id::DatabaseId,
        taggables::{
            auto_taggable_partial::AutoTaggablePartial, taggable::TaggableVariant,
            taggable_update_partial::TaggablePartial,
        },
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct AutoTaggable {
    pub id: DatabaseId,
    /// The value of the taggable that will be automatically applied.
    pub value: String,
    pub variant: TaggableVariant,
    /// A glob to be tested against when saving files. If this glob matches the
    /// ***substring*** within the user's workspace, this tag, topic or
    /// subject will be automatically applied.
    ///
    /// This means that if your path
    /// is at `/Users/bigsexy/notes/physics/Laws_And_Theorems/Keppler'
    /// s_Law_of_Planetary_Motion.md` but your 'workspace' is set to
    /// `/Users/bigsexy/notes/`, then a valid glob to match files in this
    /// directory might look like `physics/*.{mdx,cdrm,md}`.
    pub glob: String,
    pub ctime: DateTime,
    pub utime: DateTime,
}

impl_default_crud!(AutoTaggable, AutoTaggablePartial, DatabaseId);
impl<'a> DBSchema<'a> for AutoTaggable {}

impl<'a> DBEntity<'a, DatabaseId> for AutoTaggable {
    type PartialUpdateType = AutoTaggablePartial;

    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::AutoTaggable
    }

    fn merge_keys() -> &'static [&'static str] {
        &["id"]
    }

    fn primary_key() -> &'static str {
        "id"
    }

    fn primary_value(&self) -> DatabaseId {
        self.id.clone()
    }
}

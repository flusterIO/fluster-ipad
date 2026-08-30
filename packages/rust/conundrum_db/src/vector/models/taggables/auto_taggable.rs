use std::path::Path;
use std::sync::Arc;

use conundrum::{
    ecosystem::{
        db::{
            db_traits::{
                db_entity::{DBEntity, DBSchema},
                db_field::DatabaseField,
            },
            tables::DatabaseTable,
        },
        error_handling::db_error::{DatabaseError, DatabaseResult},
    },
    impl_default_crud,
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use globset::{Glob, GlobMatcher};
use serde::{Deserialize, Serialize};

use crate::vector::models::taggables::{
    auto_taggable_partial::AutoTaggablePartial, taggable::TaggableVariant, taggable_update_partial::TaggablePartial,
};
use specta::Type;

#[derive(Serialize, Deserialize, Clone, Debug, Type, Dummy, DatabaseEntity)]
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

impl AutoTaggable {
    fn get_matcher(&self) -> DatabaseResult<GlobMatcher> {
        let g = Glob::new(self.glob.as_str()).map_err(|e| {
                    log::error!("Invalid Glob found in an auto-taggable setting. This auto-taggable can't be applied.");
                    DatabaseError::InvalidGlob(self.glob.clone())
                })?
                .compile_matcher();
        Ok(g)
    }

    pub fn matches_path<P>(&self, path: P) -> DatabaseResult<bool>
        where P: AsRef<Path> {
        self.get_matcher().map(|g| g.is_match(path))
    }
}

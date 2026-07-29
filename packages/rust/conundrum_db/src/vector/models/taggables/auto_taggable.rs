use conundrum::ecosystem::db::tables::DatabaseTable;
use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

use crate::vector::{
    database::{
        db_traits::{database_field::DatabaseField, pure_model_static::PureModelStaticMethods},
        primitive_field_schema_generators::string_field_def_generator::{
            optional_clamped_float_field_definition, optional_float_field_definition, optional_string_field_definition,
            string_field_definition,
        },
    },
    models::{date_time::date_time::DateTime, primitives::db_id::DatabaseId, taggables::taggable::TaggableVariant},
};

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
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

impl PureModelStaticMethods for AutoTaggable {
    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::AutoTaggable
    }

    fn schema() -> String {
        let tbl = Self::table();
        // TODO: Narrow down this string variant to an enum, not just a string.
        formatdoc! {"
        {}
        {}
        {}
        {}
        {}
        {}
        ", 
        DatabaseId::field_definition("id", &tbl),
        string_field_definition("value", &tbl),
        string_field_definition("variant", &tbl),
        string_field_definition("glob", &tbl),
        DateTime::field_definition("ctime", &tbl),
        DateTime::field_definition("utime", &tbl),
        }
    }
}

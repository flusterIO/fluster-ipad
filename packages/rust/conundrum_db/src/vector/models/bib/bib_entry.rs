use conundrum::ecosystem::db::tables::DatabaseTable;
use indoc::formatdoc;
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::{
        db_traits::{database_field::DatabaseField, pure_model_static::PureModelStaticMethods},
        primitive_field_schema_generators::string_field_def_generator::{
            boolean_field_definition, string_field_definition,
        },
    },
    models::date_time::date_time::DateTime,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct BibEntryModel {
    /// The key of the biblatex entry, used as an id in the database as well.
    pub key: String,
    /// The raw biblatex string for a single entry.
    pub biblatex: String,
    /// A boolean indicating if this literature was already reviewed by the
    /// user.
    pub read: bool,
    pub ctime: DateTime,
    pub utime: DateTime,
}

impl PureModelStaticMethods for BibEntryModel {
    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::BibEntry
    }

    fn schema() -> String {
        let tbl = Self::table();
        formatdoc! {"
        {}
        {}
        {}
        {}
        {}
            ", string_field_definition("key", &tbl),
            string_field_definition("biblatex", &tbl),
            boolean_field_definition("read", &tbl),
            DateTime::field_definition("ctime", &tbl),
            DateTime::field_definition("utime", &tbl),
        }
    }
}

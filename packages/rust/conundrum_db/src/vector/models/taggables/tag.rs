use conundrum::ecosystem::db::tables::DatabaseTable;
use indoc::formatdoc;
use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

use crate::vector::{
    database::db_traits::{database_field::DatabaseField, pure_model_static::PureModelStaticMethods},
    models::{
        date_time::date_time::DateTime, primitives::case_insensitive_string::CaseInsensitiveString,
        taggables::tag_location::TagLocation,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct Tag {
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ctime: DateTime,
}

impl PureModelStaticMethods for Tag {
    fn schema() -> String {
        let tbl = Self::table();
        formatdoc! {"
        {}
        {}
        {}
        ", CaseInsensitiveString::field_definition("value", &tbl), TagLocation::field_definition("location", &tbl), DateTime::field_definition("ctime", &tbl)}
    }

    fn table() -> DatabaseTable {
        DatabaseTable::Tag
    }
}

// impl PureModelInstanceMethods for Tag {
//     fn save_self(&self) -> DatabaseResult<()> {
//         todo!()
//     }
// }

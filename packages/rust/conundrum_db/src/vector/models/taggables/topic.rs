use conundrum::ecosystem::{db::tables::DatabaseTable, error_handling::db_error::DatabaseResult};
use indoc::formatdoc;
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::{
        database_field::DatabaseField, pure_model_instance::PureModelInstanceMethods,
        pure_model_static::PureModelStaticMethods,
    },
    models::{
        date_time::date_time::DateTime, primitives::case_insensitive_string::CaseInsensitiveString,
        taggables::tag_location::TagLocation,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Topic {
    pub value: CaseInsensitiveString,
    pub location: TagLocation,
    pub ctime: DateTime,
}

impl PureModelStaticMethods for Topic {
    fn schema() -> String {
        let tbl = Self::table();
        formatdoc! {"
        {}
        {}
        {}
        ", CaseInsensitiveString::field_definition("value", &tbl), TagLocation::field_definition("location", &tbl), DateTime::field_definition("ctime", &tbl)}
    }

    fn table() -> DatabaseTable {
        DatabaseTable::Topic
    }
}

impl PureModelInstanceMethods for Topic {
    fn save_self(&self) -> DatabaseResult<()> {
        todo!()
    }
}

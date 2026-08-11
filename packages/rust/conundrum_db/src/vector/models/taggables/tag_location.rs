use fake::Dummy;
use lancedb::arrow::arrow_schema::Field;
use serde::{Deserialize, Serialize};

use specta::Type;

use crate::vector::database::db_traits::db_field::{DatabaseField, DatabaseFieldRepresentation};

#[derive(strum_macros::Display, Serialize, Deserialize, Clone, Debug, Dummy, Type)]
pub enum TagLocation {
    #[strum(to_string = "front_matter")]
    #[serde(rename = "front_matter")]
    FrontMatter,
    #[strum(to_string = "body")]
    #[serde(rename = "body")]
    Body,
    /// For apps using Conundrum content, this might come from a panel, a modal
    /// or what-not, but not from the note itself. If it comes from the note or
    /// front-matter, use those fields so they can be removed strategically.
    #[strum(to_string = "app_inserted")]
    #[serde(rename = "app_inserted")]
    AppInserted,
    #[strum(to_string = "auto_taggable")]
    #[serde(rename = "auto_taggable")]
    AutoTaggable,
    #[strum(to_string = "straggling")]
    #[serde(rename = "straggling")]
    /// Straggling when a tag is inserted through the REST api or another means
    /// where it is user-defined, but not necessarily associated with a
    /// note. These will never be automatically cleaned up as part of the
    /// syncing process.
    Straggling,
}

impl DatabaseField for TagLocation {
    fn field_definition(field_key: &'static str, nullable: bool) -> lancedb::arrow::arrow_schema::Field {
        Field::new(field_key, lancedb::arrow::arrow_schema::DataType::Utf8, nullable)
    }
}

impl DatabaseFieldRepresentation<String> for TagLocation {
    fn to_db_representation(&self) -> String {
        self.to_string()
    }
}

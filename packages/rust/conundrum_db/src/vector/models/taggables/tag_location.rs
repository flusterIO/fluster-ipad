use serde::{Deserialize, Serialize};

use conundrum::ecosystem::db::{
    tables::DatabaseTable, traits::database_field_representable::DatabaseFieldRepresentable,
};
use surrealdb::types::SurrealValue;

use crate::vector::database::db_traits::database_field::DatabaseField;

#[derive(strum_macros::Display, Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub enum TagLocation {
    #[strum(to_string = "front_matter")]
    #[surreal(rename = "front_matter")]
    FrontMatter,
    #[strum(to_string = "body")]
    #[surreal(rename = "body")]
    Body,
    /// For apps using Conundrum content, this might come from a panel, a modal
    /// or what-not, but not from the note itself. If it comes from the note or
    /// front-matter, use those fields so they can be removed strategically.
    #[strum(to_string = "app_inserted")]
    #[surreal(rename = "app_inserted")]
    AppInserted,
}

impl DatabaseField for TagLocation {
    fn field_definition(field_key: &'static str, table: &DatabaseTable) -> String {
        format!("DEFINE FIELD IF NOT EXISTS {} ON {} TYPE string;", field_key, table)
    }
}

impl DatabaseFieldRepresentable<String> for TagLocation {
    fn to_db_representation(&self) -> String {
        self.to_string()
    }
}

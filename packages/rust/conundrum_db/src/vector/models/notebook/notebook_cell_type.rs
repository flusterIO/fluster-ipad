use conundrum::ecosystem::db::db_traits::db_field::{DatabaseField, DatabaseFieldLarge};
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, specta::Type, Dummy, strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum NotebookCellType {
    Code,
    Markdown,
}

impl DatabaseField for NotebookCellType {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}

impl DatabaseFieldLarge for NotebookCellType {
    fn field_definition_large(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition(field_key, nullable)
    }
}

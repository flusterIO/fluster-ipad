use arrow_schema::Field;
use conundrum::ecosystem::db::db_traits::db_field::DatabaseField;
use serde::{Deserialize, Serialize};

/// TODO
///
/// ### Extraction to-do
///
/// - [ ] Extract text via conversion to html -> markdown. Currently just
///   chunking as typst.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TypstContent(String);

impl DatabaseField for TypstContent {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field {
        String::field_definition(field_key, nullable)
    }
}

impl From<String> for TypstContent {
    fn from(value: String) -> Self {
        Self(value)
    }
}

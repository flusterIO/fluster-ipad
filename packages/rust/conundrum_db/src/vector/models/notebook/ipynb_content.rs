use arrow_schema::Field;
use conundrum::{
    ecosystem::db::db_traits::db_field::{DatabaseField, DatabaseFieldLarge},
    lang::runtime::run_conundrum::ParseConundrumOptions,
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::text::text_based_content::text_based_content_trait::TextBasedContent;

/// ## To-Do
///
/// - [ ] Extract text from markdown cells and chunk.
/// - [ ] Extract chunks from code cells once chunking other code is in place.
#[derive(Serialize, Deserialize, Clone, Debug, Dummy)]
pub struct IpynbContent(String);

impl DatabaseField for IpynbContent {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field {
        String::field_definition_large(field_key, nullable)
    }
}

impl From<String> for IpynbContent {
    fn from(value: String) -> Self {
        Self(value)
    }
}

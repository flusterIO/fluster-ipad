use arrow_schema::Field;
use conundrum::ecosystem::db::db_traits::db_field::{DatabaseField, DatabaseFieldLarge};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::text::{
    text_based_content::text_based_content_trait::TextBasedContent, typst::parse_typst_opts::ParseTypstOpts,
};

/// TODO
///
/// ### Extraction to-do
///
/// - [ ] Extract text via conversion to html -> markdown. Currently just
///   chunking as typst.
#[derive(Serialize, Deserialize, Clone, Debug, Dummy, specta::Type)]
pub struct TypstContent(String);

impl DatabaseField for TypstContent {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field {
        String::field_definition_large(field_key, nullable)
    }
}

impl DatabaseFieldLarge for TypstContent {
    fn field_definition_large(field_key: &'static str, nullable: bool) -> Field {
        String::field_definition_large(field_key, nullable)
    }
}

impl From<String> for TypstContent {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl TextBasedContent<ParseTypstOpts> for TypstContent {
    fn inner_text(&self) -> String {
        self.0.to_string()
    }

    async fn get_parsed_content(&self,
                                opts: ParseTypstOpts)
                                -> conundrum::ecosystem::error_handling::ai_error::AIResult<String> {
        todo!()
    }

    async fn get_title(&self,
                       modifiers: Vec<conundrum::lang::runtime::state::parse_state::ConundrumModifier>,
                       target: conundrum::lang::runtime::state::parse_state::ConundrumCompileTarget)
                       -> conundrum::ecosystem::error_handling::ai_error::AIResult<Option<String>> {
        todo!()
    }
}

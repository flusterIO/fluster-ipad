use arrow_schema::Field;
use conundrum::{
    ecosystem::db::db_traits::db_field::{DatabaseField, DatabaseFieldLarge},
    lang::runtime::run_conundrum::ParseConundrumOptions,
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    notebook::{ipynb_parse_params::IpynbParseParameters, notebook_cell_chunk::NotebookCellChunk},
    text::text_based_content::text_based_content_trait::TextBasedContent,
};

/// ## To-Do
///
/// - [ ] Extract text from markdown cells and chunk.
/// - [ ] Extract chunks from code cells once chunking other code is in place.
#[derive(Serialize, Deserialize, Clone, Debug, Dummy, specta::Type)]
#[serde(transparent)]
pub struct IpynbContent(String);

impl DatabaseFieldLarge for IpynbContent {
    fn field_definition_large(field_key: &'static str, nullable: bool) -> Field {
        String::field_definition_large(field_key, nullable)
    }
}

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

impl TextBasedContent<IpynbParseParameters, NotebookCellChunk> for IpynbContent {
    fn inner_text(&self) -> String {
        self.0.clone()
    }

    async fn get_parsed_content(&self,
                                opts: IpynbParseParameters)
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

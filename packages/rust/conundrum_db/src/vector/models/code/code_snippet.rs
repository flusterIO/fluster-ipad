use conundrum::parsers::markdown::code_block::supported_languages::SupportedCodeBlockSyntax;

use crate::vector::models::{primitives::db_id::DatabaseId, taggables::taggables::Taggables};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, specta::Type)]
pub struct CodeSnippet {
    pub id: DatabaseId,
    pub language: Option<SupportedCodeBlockSyntax>,
    pub content: String,
    pub taggables: Taggables,
}

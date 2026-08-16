use arrow_schema::Field;
use conundrum::{
    ecosystem::error_handling::db_error::DatabaseError,
    lang::runtime::queries::get_title::get_title_group,
    lang::runtime::run_conundrum::{ParseConundrumOptions, run_conundrum},
};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use text_splitter::ChunkConfig;

use crate::vector::{
    ai_utils::ai_traits::chunk_temporary::ChunkTemporary, database::db_traits::db_field::DatabaseField, models::{primitives::db_id::DatabaseId, text::text_based_content::{text_based_chunk::TextBasedChunk, text_based_content_trait::TextBasedContent}}
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy)]
pub struct CdrmContent(String);

impl DatabaseField for CdrmContent {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field {
        String::field_definition(field_key, nullable)
    }
}

impl From<String> for CdrmContent {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl TextBasedContent<ParseConundrumOptions> for CdrmContent {
    async fn get_parsed_content(&self,
                                opts: ParseConundrumOptions)
                                -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<String> {
        let x = run_conundrum(opts).map_err(|e| {
                                       log::error!("Fail to parse Conundrum content: {:#?}", e);
                                       DatabaseError::ConundrumError(e)
                                   })?;
        Ok(x.content)
    }

    async fn get_title(&self,
                       modifiers: Vec<conundrum::lang::runtime::state::parse_state::ConundrumModifier>,
                       target: conundrum::lang::runtime::state::parse_state::ConundrumCompileTarget)
                       -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Option<String>> {
        let r = get_title_group(self.0.clone(), modifiers, target).map_err(|e| {
                    log::error!("Failed to get Conundrum title: {:#?}", e);
                    DatabaseError::ConundrumError(e)
                })?;
        if r.title.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(r.title))
        }
    }
}

impl ChunkTemporary<ParseConundrumOptions> for CdrmContent {
    async fn try_chunk_temporary(&self, opts: ParseConundrumOptions) -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<TextBasedChunk>> {
        if let Some(note_id) = &opts.note_id && !note_id.is_empty() {
        let opts = run_conundrum(ParseConundrumOptions { 
            target: conundrum::lang::runtime::state::parse_state::ConundrumCompileTarget::Markdown,
            ..opts.clone()
        }).map_err(DatabaseError::ConundrumError)?;
        let x = text_splitter::MarkdownSplitter::new(512);
        let mut chunks: Vec<TextBasedChunk> = Vec::new();
        for (i, k) in x.chunks(&opts.content).enumerate() {
            chunks.push(TextBasedChunk { document_id: DatabaseId::new_from_input_id(note_id.clone()), content: k.to_string(), chunk_idx: i as u32 });
        }
        Ok(chunks)
        } else {
            return Err(DatabaseError::FailToSerialize("a `note_id` field is required when chunking Conundrum content.".to_string()));
        }
    }
}

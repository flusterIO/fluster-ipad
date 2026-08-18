use std::ops::{Index, IndexMut};

use arrow_schema::Field;
use conundrum::{
    ai::rig::ai_traits::{ai_client_container::AIClientEmbedder, chunk_temporary::ChunkTemporary}, ecosystem::{db::db_traits::db_field::DatabaseField, error_handling::{ai_error::{AIError, AIResult}, db_error::DatabaseError}}, lang::runtime::{queries::get_title::get_title_group, run_conundrum::{ParseConundrumOptions, run_conundrum}}, lifted_models::primitives::db_id::DatabaseId
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::{
models::{text::text_based_content::{text_based_chunk::TextBasedChunk, text_based_content_trait::TextBasedContent}, vector::vector::DBVector}
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

impl<ClientType> ChunkTemporary<ParseConundrumOptions, TextBasedChunk, ClientType> for CdrmContent where ClientType: AIClientEmbedder<String> {

    async fn try_chunk_temporary(&self, opts: ParseConundrumOptions, client: &std::sync::Arc<tokio::sync::Mutex<ClientType>>) -> AIResult<Vec<TextBasedChunk>> {
        if let Some(note_id) = &opts.note_id && !note_id.is_empty() {
        let opts = run_conundrum(ParseConundrumOptions { 
            target: conundrum::lang::runtime::state::parse_state::ConundrumCompileTarget::Markdown,
            ..opts.clone()
        }).map_err(AIError::ConundrumError)?;
        let locked_client = client.clone().lock_owned().await;
        let x = text_splitter::MarkdownSplitter::new(512);
        let mut chunks: Vec<TextBasedChunk> = Vec::new();
        let chunk_strings = x.chunks(&opts.content).map(|v| {
               v.to_string()     
        }).collect::<Vec<String>>();
            let vectors = locked_client.embed_models(None, chunk_strings.clone(), None).await?;
            for (i, embedding) in vectors.iter().enumerate() {
                 let chunk_i = chunk_strings.index(i);
                 if *chunk_i == embedding.document {
                     let text_chunk = TextBasedChunk {
                         document_id: DatabaseId::new_from_input_id(note_id.clone()),
                         content: chunk_i.clone(),
                         chunk_idx: i as u32,
                         vector: DBVector(embedding.vec.clone())
                     };
                     chunks.push(text_chunk);
                 } else {
                     log::error!("Found an unmatched vector! Something went haywire.");
                 }
            }
        Ok(chunks)
        } else {
            return Err(AIError::InvalidProps("a `note_id` field is required when chunking Conundrum content.".to_string()));
        }
    }
}

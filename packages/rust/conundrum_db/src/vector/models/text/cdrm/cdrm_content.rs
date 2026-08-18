use std::{ops::{Index, IndexMut}, sync::Arc};

use arrow_schema::Field;
use conundrum::{
    ai::rig::ai_traits::{ai_client_container::AIClientEmbedder, chunk_temporary::Chunk}, ecosystem::{db::db_traits::db_field::DatabaseField, error_handling::{ai_error::{AIError, AIResult}, db_error::DatabaseError}}, lang::{lib::shared::utility_types::ArcTokioMutex, runtime::{queries::get_title::get_title_group, run_conundrum::{ParseConundrumOptions, run_conundrum}}}, lifted_models::primitives::db_id::DatabaseId
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{ecosystem_data::server_state::server_state::ServerState, text::text_based_content::{text_based_chunk::TextBasedChunk, text_based_content_trait::TextBasedContent}, vector::vector::DBVector};

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

impl Chunk<ParseConundrumOptions, TextBasedChunk, ServerState> for CdrmContent {
    async fn try_chunk(&self,
        opts: ParseConundrumOptions,
        locked_state: &Arc<ServerState>)
        -> AIResult<(AIResult<Vec<TextBasedChunk>>, AIResult<Vec<TextBasedChunk>>)> {
                let opts = run_conundrum(ParseConundrumOptions { 
                    target: conundrum::lang::runtime::state::parse_state::ConundrumCompileTarget::Markdown,
                    ..opts.clone()
                }).map_err(AIError::ConundrumError)?;

                let x = text_splitter::MarkdownSplitter::new(512);
                let chunk_strings = x.chunks(&opts.content).map(|v| {
                    v.to_string()     
                }).collect::<Vec<String>>();
                let local_vectors = match &locked_state.local_client {
                    Some(s) => {
                        let client = s.clone().lock_owned().await;
                        let r = client.embed_models(None, chunk_strings.clone(), None).await?;
                        Ok(r.iter().enumerate().map(|(i, x)| {
                            TextBasedChunk { 
                                id: DatabaseId::default(),
                                document_id: DatabaseId::new_from_input_id("Conundrum Documentation".to_string()),
                                content: chunk_strings.index(i).clone(),
                                chunk_idx: i as u32,
                                vector: DBVector(x.vec.clone())
                            }
                        }).collect::<Vec<TextBasedChunk>>())
                    } 
                    None => Err(
                        AIError::InvalidLocalProvider
                    )
                };
                let remote_vectors = match &locked_state.remote_client {
                    Some(s) => {
                        let client = s.clone().lock_owned().await;
                        let r = client.embed_models(None, chunk_strings.clone(), None).await?;
                        Ok(r.iter().enumerate().map(|(i, x)| {
                            TextBasedChunk { 
                                id: DatabaseId::default(),
                                document_id: DatabaseId::new_from_input_id("Conundrum Documentation".to_string()),
                                content: chunk_strings.index(i).clone(),
                                chunk_idx: i as u32,
                                vector: DBVector(x.vec.clone())
                            }
                        }).collect::<Vec<TextBasedChunk>>())
                    } 
                    None => Err(
                        AIError::InvalidRemoteProvider
                    )
                };
                Ok((local_vectors, remote_vectors))
        }
}

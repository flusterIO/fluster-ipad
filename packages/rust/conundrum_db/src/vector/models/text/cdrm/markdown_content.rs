use std::{ops::Index, sync::Arc};

use conundrum::{
    ai::{
        agents::remote_agent,
        models::chat::vector::vector_model::DBVector,
        rig::ai_traits::{ai_client_container::AIClientEmbedder, chunk::Chunk},
    },
    ecosystem::{
        db::db_traits::db_field::{DatabaseField, DatabaseFieldLarge},
        error_handling::ai_error::{AIError, AIResult},
    },
    lang::runtime::{
        queries::get_title::get_title_group,
        run_conundrum::{ParseConundrumOptions, run_conundrum},
    },
    lifted_models::primitives::db_id::DatabaseId,
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    ecosystem_data::server_state::server_state::ServerState,
    text::text_based_content::{text_based_chunk::TextBasedChunk, text_based_content_trait::TextBasedContent},
};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy)]
pub struct MarkdownContent(pub String);

impl DatabaseField for MarkdownContent {
    fn field_definition(field_key: &'static str, nullable: bool) -> arrow_schema::Field {
        String::field_definition_large(field_key, nullable)
    }
}

impl From<String> for MarkdownContent {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl TextBasedContent<ParseConundrumOptions> for MarkdownContent {
    async fn get_parsed_content(&self, opts: ParseConundrumOptions) -> AIResult<String> {
        let x = run_conundrum(opts).map_err(|e| {
                                       log::error!("Fail to parse Conundrum content: {:#?}", e);
                                       AIError::ConundrumError(e)
                                   })?;
        Ok(x.content)
    }

    async fn get_title(&self,
                       modifiers: Vec<conundrum::lang::runtime::state::parse_state::ConundrumModifier>,
                       target: conundrum::lang::runtime::state::parse_state::ConundrumCompileTarget)
                       -> AIResult<Option<String>> {
        let r = get_title_group(self.0.clone(), modifiers, target).map_err(|e| {
                    log::error!("Failed to get Conundrum title: {:#?}", e);
                    AIError::ConundrumError(e)
                })?;
        if r.title.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(r.title))
        }
    }
}

impl Chunk<ParseConundrumOptions, TextBasedChunk, ServerState> for MarkdownContent {
    async fn try_chunk(&self,
                       opts: ParseConundrumOptions,
                       locked_state: Arc<ServerState>)
                       -> AIResult<Vec<TextBasedChunk>> {
        let x = text_splitter::MarkdownSplitter::new(512);
        let chunk_strings = x.chunks(self.0.as_str()).map(|v| v.to_string()).collect::<Vec<String>>();
        match &locked_state.local_client {
            Some(s) => {
                let client = s.clone().lock_owned().await;
                let r = client.embed_models(None, chunk_strings.clone(), None).await?;

                let remote_vectors = match &locked_state.remote_client {
                                         Some(s) => {
                                             let client = s.clone().lock_owned().await;
                                             let r = client.embed_models(None, chunk_strings.clone(), None).await?;
                                             Ok(r)
                                         }
                                         None => Err(AIError::InvalidRemoteProvider),
                                     }.ok();
                Ok(r.iter()
                    .enumerate()
                    .map(|(i, x)| {
                        TextBasedChunk { id: DatabaseId::default(),
                                         document_id:
                                             DatabaseId::new_from_input_id("Conundrum Documentation".to_string()),
                                         content: chunk_strings.index(i).clone(),
                                         chunk_idx: i as u32,
                                         local_vector: DBVector(x.vec.clone()),
                                         remote_vector: remote_vectors.as_ref().cloned().map(|k| {
                                                                                            DBVector(k.index(i)
                                                                                                      .vec
                                                                                                      .clone())
                                                                                        }) }
                    })
                    .collect::<Vec<TextBasedChunk>>())
            }
            None => Err(AIError::InvalidLocalProvider),
        }
    }
}

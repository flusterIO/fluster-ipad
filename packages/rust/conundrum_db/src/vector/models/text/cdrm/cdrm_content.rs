use std::{ops::{Index, IndexMut}, sync::Arc};

use arrow_schema::Field;
use conundrum::{
    ai::{models::chat::vector::vector_model::DBVector, rig::ai_traits::{ai_client_container::AIClientEmbedder, chunk::Chunk}}, ecosystem::{db::db_traits::db_field::{DatabaseField, DatabaseFieldLarge}, error_handling::{ai_error::{AIError, AIResult}, db_error::DatabaseError}}, lang::{lib::shared::utility_types::ArcTokioMutex, runtime::{queries::get_title::get_title_group, run_conundrum::{ParseConundrumOptions, run_conundrum}}}, lifted_models::primitives::db_id::DatabaseId
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{ecosystem_data::server_state::server_state::ServerState, text::{cdrm::markdown_content::MarkdownContent, text_based_content::{text_based_chunk::TextBasedChunk, text_based_content_trait::TextBasedContent}}};

#[derive(Serialize, Deserialize, Clone, Debug, Dummy, specta::Type)]
pub struct CdrmContent(pub String);

impl DatabaseFieldLarge for CdrmContent {
    fn field_definition_large(field_key: &'static str, nullable: bool) -> Field {
        String::field_definition_large(field_key, nullable)
    }
}

impl DatabaseField for CdrmContent {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field {
        String::field_definition_large(field_key, nullable)
    }
}

impl From<String> for CdrmContent {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<CdrmContent> for String {
    fn from(value: CdrmContent) -> Self {
        value.inner_text()
    }
}

impl TextBasedContent<ParseConundrumOptions> for CdrmContent {
    fn inner_text(&self) -> String {
        self.0.clone()
    }
    async fn get_parsed_content(&self,
        opts: ParseConundrumOptions)
        -> AIResult<String> {
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

impl Chunk<ParseConundrumOptions, TextBasedChunk, ServerState> for CdrmContent {
    async fn try_chunk(&self,
        opts: ParseConundrumOptions,
        locked_state: Arc<ServerState>)
        -> AIResult<Vec<TextBasedChunk>> {
                let res = run_conundrum(ParseConundrumOptions { 
                    target: conundrum::lang::runtime::state::parse_state::ConundrumCompileTarget::Markdown,
                    ..opts.clone()
                }).map_err(AIError::ConundrumError)?;
            let md = MarkdownContent(res.content);
            md.try_chunk(opts.clone(), Arc::clone(&locked_state)).await
        }
}

use std::sync::Arc;

use arrow_schema::Field;
use conundrum::{
    ai::rig::ai_traits::chunk::Chunk,
    ecosystem::{
        db::db_traits::db_field::{DatabaseField, DatabaseFieldLarge},
        error_handling::ai_error::{AIError, AIResult},
    },
    lang::runtime::{queries::get_title::get_title_group, run_conundrum::ParseConundrumOptions},
};
use html_to_markdown_rs::{ConversionOptions, convert};
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    ecosystem_data::server_state::server_state::ServerState,
    text::{
        cdrm::cdrm_content::CdrmContent,
        text_based_content::{text_based_chunk::TextBasedChunk, text_based_content_trait::TextBasedContent},
    },
};

/// TODO
///
/// ### Extraction to-do
/// - [x] Extract text as markdown
/// - [ ] Extract tables
/// - [ ] Extract images
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HTMLContent(String);

impl DatabaseField for HTMLContent {
    fn field_definition(field_key: &'static str, nullable: bool) -> Field {
        String::field_definition_large(field_key, nullable)
    }
}

impl From<String> for HTMLContent {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl TextBasedContent<()> for HTMLContent {
    fn inner_text(&self) -> String {
        self.0.clone()
    }

    async fn get_parsed_content(&self, _: ()) -> AIResult<String> {
        let res = convert(&self.0, ConversionOptions::default()).map_err(|e| {
                                                                    log::error!("HTML Conversion error: {:#?}", e);
                                                                    AIError::FailToExtractText("HTML".to_string())
                                                                })?;
        let content = res.content.ok_or_else(|| AIError::FailToExtractText("HTML".to_string()))?;
        Ok(content)
    }

    async fn get_title(&self,
                       modifiers: Vec<conundrum::lang::runtime::state::parse_state::ConundrumModifier>,
                       target: conundrum::lang::runtime::state::parse_state::ConundrumCompileTarget)
                       -> AIResult<Option<String>> {
        let content = self.get_parsed_content(()).await.map_err(|e| {
                                                            log::error!("Error: {:#?}", e);
                                                            AIError::FailToExtractText("html".to_string())
                                                        })?;
        let r = get_title_group(content, modifiers, target).map_err(|e| {
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

impl Chunk<ParseConundrumOptions, TextBasedChunk, ServerState> for HTMLContent {
    async fn try_chunk(&self, opts: ParseConundrumOptions, state: Arc<ServerState>) -> AIResult<Vec<TextBasedChunk>> {
        let content = self.get_parsed_content(()).await?;
        let cdrm = CdrmContent::from(content);
        cdrm.try_chunk(opts, Arc::clone(&state)).await
    }
}

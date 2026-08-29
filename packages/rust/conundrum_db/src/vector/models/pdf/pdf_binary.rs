use std::sync::Arc;

use conundrum::{
    ai::rig::ai_traits::chunk::Chunk,
    ecosystem::error_handling::{
        ai_error::{AIError, AIResult},
        db_error::DatabaseError,
    },
    lang::runtime::{queries::get_title::get_title_group, run_conundrum::ParseConundrumOptions},
};
use pdf_oxide::{
    PdfDocument,
    pipeline::{MarkdownOutputConverter, OutputConverter, TextPipeline, TextPipelineConfig},
};
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    binary::{binary::Binary, binary_based_content_trait::BinaryBasedContent},
    ecosystem_data::server_state::server_state::ServerState,
    text::{cdrm::cdrm_content::CdrmContent, text_based_content::text_based_chunk::TextBasedChunk},
};

/// ## TO-DO
/// - [x] Extract text as markdown
/// - [x] Chunk extracted text
/// - [ ] Extract images
/// - [ ] Extract tables
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, specta::Type, fake::Dummy)]
pub struct PdfBinary(Binary);

impl BinaryBasedContent<ParseConundrumOptions> for PdfBinary {
    fn bytes(&self) -> Vec<u8> {
        self.0.bytes()
    }

    async fn get_parsed_content(&self, opts: ParseConundrumOptions) -> AIResult<String> {
        let mut doc = PdfDocument::from_bytes(self.bytes()).map_err(|e| {
                                                               log::error!("Database: {:#?}", e);
                                                               AIError::FailToExtractText("pdf".to_string())
                                                           })?;
        let config = TextPipelineConfig::default();
        let spans = doc.extract_spans(0).map_err(|e| {
                                             log::error!("Database: {:#?}", e);
                                             AIError::FailToExtractText("pdf".to_string())
                                         })?;
        let pipeline = TextPipeline::with_config(config.clone());
        let ordered_spans =
            pipeline.process(spans, Default::default()).map_err(|e| {
                                                            log::error!("Database: {:#?}", e);
                                                            AIError::FailToExtractText("pdf".to_string())
                                                        })?;
        let converter = MarkdownOutputConverter::new();
        let markdown = converter.convert(&ordered_spans, &config).map_err(|e| {
                                                                      log::error!("Database: {:#?}", e);
                                                                      AIError::FailToExtractText("pdf".to_string())
                                                                  })?;
        Ok(markdown)
    }

    async fn get_title(&self,
                       modifiers: Vec<conundrum::lang::runtime::state::parse_state::ConundrumModifier>,
                       target: conundrum::lang::runtime::state::parse_state::ConundrumCompileTarget)
                       -> AIResult<Option<String>> {
        let content =
            self.get_parsed_content(ParseConundrumOptions::for_chunking()).await.map_err(|e| {
                                                                                     log::error!("Error: {:#?}", e);
                                                                                    AIError::FailToExtractText("pdf".to_string())
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

impl Chunk<ParseConundrumOptions, TextBasedChunk, ServerState> for PdfBinary {
    async fn try_chunk(&self,
                       opts: ParseConundrumOptions,
                       state: std::sync::Arc<ServerState>)
                       -> Result<Vec<TextBasedChunk>, AIError> {
        let parsed_content = self.get_parsed_content(opts.clone()).await?;
        let cdrm = CdrmContent::from(parsed_content);
        cdrm.try_chunk(opts, Arc::clone(&state)).await
    }
}

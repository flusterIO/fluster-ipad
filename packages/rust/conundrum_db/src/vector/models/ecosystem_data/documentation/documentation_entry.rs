use conundrum::{
    ai::rig::ai_traits::chunk_temporary::Chunk, ecosystem::error_handling::ai_error::AIError,
    lang::runtime::run_conundrum::ParseConundrumOptions,
};
use std::sync::Arc;

use crate::vector::models::{
    ecosystem_data::{documentation::documentation_key::DocumentationKey, server_state::server_state::ServerState},
    text::{cdrm::cdrm_content::CdrmContent, text_based_content::text_based_chunk::TextBasedChunk},
};

pub struct DocumentationEntry {
    pub key: DocumentationKey,
}

impl Chunk<ParseConundrumOptions, TextBasedChunk, ServerState> for DocumentationEntry {
    async fn try_chunk(
        &self,
        opts: ParseConundrumOptions,
        state: &Arc<ServerState>)
        -> Result<(Result<Vec<TextBasedChunk>, conundrum::ecosystem::error_handling::ai_error::AIError>,
                   Result<Vec<TextBasedChunk>, conundrum::ecosystem::error_handling::ai_error::AIError>),
                  AIError> {
        let x: String = self.key.clone().into();
        let y: CdrmContent = x.into();
        let res = y.try_chunk(opts, state).await?;
        Ok(res)
    }
}

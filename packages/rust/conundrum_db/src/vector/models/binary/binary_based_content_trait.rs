use std::path::Path;

use conundrum::{
    ecosystem::error_handling::{
        conundrum_fs_error::ConundrumFSError,
        db_error::{DatabaseError, DatabaseResult},
    },
    lang::runtime::state::parse_state::{ConundrumCompileTarget, ConundrumModifier},
};

use crate::vector::models::text::text_based_content::text_based_chunk::TextBasedChunk;

pub trait BinaryBasedContent<ParseParameters, ChunkType = TextBasedChunk> {
    fn bytes(&self) -> Vec<u8>;
    /// Returns mardown or text via some extraction method like OCR or whatnot
    async fn get_parsed_content(&self, opts: ParseParameters) -> DatabaseResult<String>;
    async fn get_title(&self,
                       modifiers: Vec<ConundrumModifier>,
                       target: ConundrumCompileTarget)
                       -> DatabaseResult<Option<String>>;
}

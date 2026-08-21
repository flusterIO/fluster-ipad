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
    fn abs_path(&self) -> impl AsRef<Path>;
    async fn read(&self) -> DatabaseResult<Vec<u8>> {
        let fp = self.abs_path();
        let y = tokio::fs::read(fp).await.map_err(|e| {
                                              log::error!("Error trying to read file: {:#?}", e);
                                              DatabaseError::FileSystemError(ConundrumFSError::FsError(e.to_string()))
                                          })?;
        Ok(y)
    }
    /// Returns mardown or text via some extraction method like OCR or whatnot
    async fn get_parsed_content(&self, opts: ParseParameters) -> DatabaseResult<String>;
    async fn get_title(&self,
                       modifiers: Vec<ConundrumModifier>,
                       target: ConundrumCompileTarget)
                       -> DatabaseResult<Option<String>>;
}

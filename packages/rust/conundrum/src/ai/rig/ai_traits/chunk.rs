use crate::ecosystem::error_handling::db_error::DatabaseResult;

pub trait Chunk<ParseParameters, ChunkType> {
    async fn try_chunk(&self, opts: ParseParameters) -> DatabaseResult<Vec<ChunkType>>;
}

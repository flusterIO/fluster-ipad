use crate::lang::runtime::chunk_config::{ArcRwLockChunkOpts, ChunkConundrumOptions};

use crate::lang::runtime::state::conundrum_error_variant::ConundrumResult;

pub trait Chunkable {
    fn chunks(&self, cfg: &ArcRwLockChunkOpts) -> Vec<String>;
}

pub trait TryChunkable {
    fn try_chunks(&self, cfg: &ArcRwLockChunkOpts) -> ConundrumResult<Vec<String>>;
}

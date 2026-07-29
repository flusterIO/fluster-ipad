use std::sync::Arc;

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

pub fn default_max_chunk() -> u32 {
    500
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChunkConundrumOptions {
    #[serde(default = "default_max_chunk")]
    pub max_chunk_size: u32,
}

pub type ArcRwLockChunkOpts = Arc<RwLock<ChunkConundrumOptions>>;

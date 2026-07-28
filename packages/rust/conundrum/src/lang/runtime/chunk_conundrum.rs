use serde::{Deserialize, Serialize};

use crate::lang::runtime::state::conundrum_error_variant::ConundrumResult;

pub fn default_max_chunk() -> u32 {
    500
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChunkConundrumOptions {
    #[serde(default = "default_max_chunk")]
    pub max_chunk_size: u32,
}

// pub fn chunk_conundrum(opts: ChunkConundrumOptions) ->
// ConundrumResult<Vec<String>> {

// }

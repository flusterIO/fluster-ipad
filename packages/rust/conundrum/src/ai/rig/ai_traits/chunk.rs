use std::sync::Arc;

use crate::{
    ai::rig::ai_traits::ai_client_container::AIClientEmbedder, ecosystem::error_handling::ai_error::AIResult,
    lang::lib::shared::utility_types::ArcTokioMutex,
};

pub trait Chunk<ParseParameters, ChunkType, ServerStateType> {
    /// Returns (Local, Remote) chunks un that order.
    async fn try_chunk(&self,
                       opts: ParseParameters,
                       state: &Arc<ServerStateType>)
                       -> AIResult<(AIResult<Vec<ChunkType>>, AIResult<Vec<ChunkType>>)>;
}

use std::sync::Arc;

use conundrum::{
    ai::rig::ai_traits::{ai_client_container::AIClientEmbedder, chunk::Chunk},
    ecosystem::{
        db::{
            db_client::db_client::DBClient,
            db_traits::{db_entity::DBSchema, entity_crud::EntityCRUD, local_default::DefaultLocalVectorGeneration},
            parameters::ai::schema_parameters::SchemaParameters,
            tables::DatabaseTable,
        },
        error_handling::db_error::{DatabaseError, DatabaseResult},
    },
    lifted_models::primitives::db_id::DatabaseId,
};
use serde::Serialize;

pub trait SeedContent: Default {
    async fn try_seed<'a>(&self, db: DBClient) -> DatabaseResult<()>;
}

pub trait SeedChunks<'a, ChunkType, PartialUpdateType, ParseParameters, ServerStateType>:
    Chunk<ParseParameters, ChunkType, ServerStateType>
    where ChunkType: DBSchema + EntityCRUD<PartialUpdateType> + Clone + Serialize,
          PartialUpdateType: Clone + DBSchema + Serialize {
    fn table() -> DatabaseTable;
    async fn try_seed(&self, db: DBClient, opts: ParseParameters, state: Arc<ServerStateType>) -> DatabaseResult<()> {
        let chunks = self.try_chunk(opts, Arc::clone(&state)).await.map_err(|e| {
                                                                        log::error!("AI Error: {:#?}", e);
                                                                        DatabaseError::AIError(e)
                                                                    })?;
        ChunkType::save_many(chunks, db).await.inspect_err(|e| {
                                                   log::error!("Failed to save seed content: {:#?}", e);
                                               })?;
        Ok(())
    }
}

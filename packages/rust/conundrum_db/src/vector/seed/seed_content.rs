use std::sync::Arc;

use conundrum::{
    ai::rig::ai_traits::{ai_client_container::AIClientEmbedder, chunk_temporary::Chunk},
    ecosystem::{
        db::{
            db::ArcMutexDB,
            db_traits::{db_entity::DBSchema, entity_crud::EntityCRUD, local_default::DefaultLocalVectorGeneration},
            parameters::ai::schema_parameters::SchemaParameters,
            tables::DatabaseTable,
        },
        error_handling::db_error::{DatabaseError, DatabaseResult},
    },
    lifted_models::primitives::db_id::DatabaseId,
};

pub trait SeedContent<'a, ChunkType, PartialUpdateType, ParseParameters, ServerStateType>:
    Chunk<ParseParameters, ChunkType, ServerStateType>
    where ChunkType: DBSchema<'a> + EntityCRUD<'a, DatabaseId, PartialUpdateType> + Clone,
          PartialUpdateType: Clone + DBSchema<'a> {
    fn table() -> DatabaseTable;
    async fn try_seed(&self,
                      db: &ArcMutexDB,
                      opts: ParseParameters,
                      agent: &Arc<ServerStateType>)
                      -> DatabaseResult<()> {
        let (local_chunks, remote_chunks) = self.try_chunk(opts, agent).await.map_err(|e| {
                                                                                  log::error!("AI Error: {:#?}", e);
                                                                                  DatabaseError::AIError(e)
                                                                              })?;
        if let Ok(lc) = local_chunks {
            // TODO: Move this to a new type and just wrap the TextBasedChunk in a macro
            // that generates the necessary new type.
            ChunkType::save_many(lc, db).await.inspect_err(|e| {
                                                   log::error!("Failed to save seed content: {:#?}", e);
                                               })?;
        } else {
            log::warn!("Could not seed locally generated vectors. You won't be able to access some AI features offline.");
        }

        if let Ok(rc) = remote_chunks {
            // TODO: Move this to a new type and just wrap the TextBasedChunk in a macro
            // that generates the necessary new type.
            ChunkType::save_many(rc, db).await.inspect_err(|e| {
                                                   log::error!("Failed to save seed content: {:#?}", e);
                                               })?;
        } else {
            log::warn!("Could not seed remotely generated vectors.");
        }
        Ok(())
    }
}

use conundrum::{
    ai::rig::ai_traits::{
        ai_client_container::AIClientEmbedder, chunk_temporary::Chunk, conundrum_agent::ConundrumAgent,
    },
    ecosystem::{
        db::{db::ArcMutexDB, db_traits::entity_crud::EntityCRUD, tables::DatabaseTable},
        error_handling::{
            ai_error::{AIError, AIResult},
            db_error::{DatabaseError, DatabaseResult},
        },
    },
    lang::runtime::run_conundrum::ParseConundrumOptions,
};
use fake::rand::seq::IndexedRandom;
use strum::IntoEnumIterator;

use crate::vector::{
    models::{
        ecosystem_data::{
            documentation::{documentation_entry::DocumentationEntry, documentation_key::DocumentationKey},
            server_state::server_state::ServerState,
        },
        text::text_based_content::text_based_chunk::TextBasedChunk,
    },
    seed::seed_content::SeedContent,
};

pub struct SeedDocumentation(Vec<DocumentationEntry>);

impl Default for SeedDocumentation {
    fn default() -> Self {
        let mut items = Vec::new();
        for k in DocumentationKey::iter() {
            items.push(DocumentationEntry { key: k });
        }
        Self(items)
    }
}

impl Chunk<ParseConundrumOptions, TextBasedChunk, ServerState> for SeedDocumentation {
    async fn try_chunk(&self,
                       opts: ParseConundrumOptions,
                       agent: &std::sync::Arc<ServerState>)
                       -> AIResult<(AIResult<Vec<TextBasedChunk>>, AIResult<Vec<TextBasedChunk>>)> {
        let mut local_res = Vec::new();
        let mut remote_res = Vec::new();
        for k in &self.0 {
            let (local_chunks, remote_chunks) = k.try_chunk(opts.clone(), agent).await?;
            if let Ok(lc) = local_chunks {
                local_res.extend(lc);
            }
            if let Ok(rc) = remote_chunks {
                remote_res.extend(rc);
            }
        }
        Ok((match local_res.len() {
                0 => Err(AIError::InvalidLocalProvider),
                _ => Ok(local_res),
            },
            match remote_res.len() {
                0 => Err(AIError::InvalidRemoteProvider),
                _ => Ok(remote_res),
            }))
    }
}

impl<'a> SeedContent<'a, TextBasedChunk, TextBasedChunk, ParseConundrumOptions, ServerState> for SeedDocumentation {
    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::DocumentationChunk
    }

    async fn try_seed(&self,
                      db: &ArcMutexDB,
                      opts: ParseConundrumOptions,
                      agent: &std::sync::Arc<ServerState>)
                      -> DatabaseResult<()> {
        let (local_chunks, remote_chunks) = self.try_chunk(opts, agent).await.map_err(DatabaseError::AIError)?;
        if let Ok(lc) = local_chunks {
            TextBasedChunk::save_many(lc, db).await.inspect_err(|e| {
                                                        log::error!("Failed to save seed content: {:#?}", e);
                                                    })?;
        } else {
            log::warn!("Failed to save local documentation chunks");
        }
        if let Ok(rc) = remote_chunks {
            TextBasedChunk::save_many(rc, db).await.inspect_err(|e| {
                                                        log::error!("Failed to save seed content: {:#?}", e);
                                                    })?;
        } else {
            log::warn!("Failed to save local documentation chunks");
        }
        Ok(())
    }
}

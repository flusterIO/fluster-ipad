use std::sync::Arc;

use conundrum::{
    ai::rig::ai_traits::{ai_client_container::AIClientEmbedder, chunk::Chunk, conundrum_agent::ConundrumAgent},
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
    seed::seed_content::SeedChunks,
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
    async fn try_chunk(&self, opts: ParseConundrumOptions, state: std::sync::Arc<ServerState>) -> Result<_, AIError> {
        let mut chunk_res = Vec::new();
        for k in &self.0 {
            let chunks = k.try_chunk(opts.clone(), Arc::clone(&state)).await?;
            chunk_res.extend(chunks);
        }
        match chunk_res.len() {
            0 => Err(AIError::InvalidLocalProvider),
            _ => Ok(chunk_res),
        }
    }
}

impl<'a> SeedChunks<'a, TextBasedChunk, TextBasedChunk, ParseConundrumOptions, ServerState> for SeedDocumentation {
    fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
        DatabaseTable::DocumentationChunk
    }

    async fn try_seed(&self,
                      db: ArcMutexDB,
                      opts: ParseConundrumOptions,
                      state: std::sync::Arc<ServerState>)
                      -> DatabaseResult<()> {
        let chunks = self.try_chunk(opts, Arc::clone(&state)).await.map_err(DatabaseError::AIError)?;
        // TextBasedChunk::save_many(lc, Arc::clone(&db)).await.inspect_err(|e| {
        //                                                          log::error!("Failed to save seed content: {:#?}", e);
        //                                                      })?;
        Ok(())
    }
}

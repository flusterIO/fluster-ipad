use conundrum::ecosystem::db::tables::DatabaseTable;
use conundrum_macros::{DBSchema, DatabaseEntity};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::text::text_based_content::text_based_chunk::TextBasedChunk;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[serde(transparent)]
#[db(table = DatabaseTable::CdrmChunk, unit = TextBasedChunk)]
pub struct CdrmChunk(TextBasedChunk);

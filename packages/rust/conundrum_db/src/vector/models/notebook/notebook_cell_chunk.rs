use conundrum::{
    ai::models::chat::vector::vector_model::DBVector, ecosystem::db::tables::DatabaseTable,
    lifted_models::primitives::db_id::DatabaseId,
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::notebook::notebook_cell_type::NotebookCellType;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::NotebookCellChunk)]
pub struct NotebookCellChunk {
    #[serde(default = "DatabaseId::new")]
    pub id: DatabaseId,
    pub content: String,
    pub cell_type: NotebookCellType,
    pub local_vector: DBVector,
    pub remote_vector: Option<DBVector>,
}

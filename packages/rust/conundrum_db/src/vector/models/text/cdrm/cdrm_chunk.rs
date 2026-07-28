use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

use crate::vector::{database::db_traits::pure_model_static::PureModelStaticMethods, models::primitives::db_id::DatabaseId};

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct CdrmChunk {
    pub content: String,
    pub note_id: DatabaseId,
    pub chunk_idx: String,
    pub vec: Vec<f64>,
}

// impl PureModelStaticMethods for CdrmChunk {
//     fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
//         conundrum::ecosystem::db::tables::DatabaseTable::CdrmVector
//     }

//     fn schema() -> String {
//         todo!()
//     }
// }

use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

use crate::vector::models::primitives::db_id::DatabaseId;

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct IDRecord {
    pub id: DatabaseId,
}

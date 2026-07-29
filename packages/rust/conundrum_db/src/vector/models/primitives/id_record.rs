use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;

use crate::vector::models::primitives::db_id::DatabaseId;

/// Deprecated: Didn't know Surreal had basically the same thing ready to go.
#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct IDRecord {
    pub id: DatabaseId,
}

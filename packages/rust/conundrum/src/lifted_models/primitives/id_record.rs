use serde::{Deserialize, Serialize};

use crate::lifted_models::primitives::db_id::DatabaseId;

/// Deprecated: Didn't know Surreal had basically the same thing ready to go.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IDRecord {
    pub id: DatabaseId,
}

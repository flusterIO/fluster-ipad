use std::sync::Arc;

use conundrum::{
    ecosystem::db::db_traits::{db_entity::DBSchema, db_field::DatabaseField},
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use conundrum_macros::{DBSchema, DatabaseEntity};
use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, DBSchema)]
pub struct AssignmentEntityPartial {
    pub id: DatabaseId,
    pub label: Option<String>,
    pub description: Option<String>,
    pub due_at: Option<DateTime>,
}

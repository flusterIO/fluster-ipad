use std::sync::Arc;

use conundrum::{
    ecosystem::db::{
        db_traits::{db_entity::DBSchema, db_field::DatabaseField},
        tables::DatabaseTable,
    },
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use conundrum_macros::DatabaseEntity;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::academic::assignment::assignment_status::AssignmentStatus;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy, DatabaseEntity)]
#[db(table = DatabaseTable::Milestone)]
pub struct MilestoneEntity {
    pub id: DatabaseId,
    pub label: String,
    pub description: Option<String>,
    pub status: AssignmentStatus,
    pub due_at: Option<DateTime>,
}

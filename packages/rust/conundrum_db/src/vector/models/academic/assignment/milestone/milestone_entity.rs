use std::sync::Arc;

use conundrum::{
    ecosystem::db::db_traits::{db_entity::DBSchema, db_field::DatabaseField},
    lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId},
};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::academic::assignment::assignment_status::AssignmentStatus;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct MilestoneEntity {
    pub id: DatabaseId,
    pub label: String,
    pub description: Option<String>,
    pub status: AssignmentStatus,
    pub due_at: Option<DateTime>,
}

impl<'a> DBSchema<'a> for MilestoneEntity {
    fn arrow_fields(
        )
        -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
    {
        Ok(vec![Arc::new(DatabaseId::field_definition("id", false)),
                Arc::new(String::field_definition("label", false)),
                Arc::new(String::field_definition("description", true)),
                Arc::new(AssignmentStatus::field_definition("status", false)),
                Arc::new(DateTime::field_definition("due_at", true)),])
    }
}

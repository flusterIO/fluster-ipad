use std::sync::Arc;

use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::db_field::DatabaseField,
    models::{
        academic::assignment::assignment_status::AssignmentStatus, date_time::date_time::DateTime,
        primitives::db_id::DatabaseId,
    },
};

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

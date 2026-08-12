use std::sync::Arc;

use conundrum::ecosystem::db::traits::db_entity::DBSchema;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::{
    database::db_traits::db_field::DatabaseField,
    models::{
        academic::assignment::assignment_status::AssignmentStatus,
        date_time::{alarm::alarm_entity::AlarmEntity, date_time::DateTime},
        primitives::db_id::DatabaseId,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct Milestone {
    pub id: DatabaseId,
    pub label: String,
    pub description: Option<String>,
    pub status: AssignmentStatus,
    pub due_at: Option<DateTime>,
    pub alarms: Vec<AlarmEntity>,
}

use std::sync::Arc;

use conundrum::lifted_models::primitives::{date_time::DateTime, db_id::DatabaseId};
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    academic::assignment::assignment_status::AssignmentStatus, date_time::alarm::alarm_entity::AlarmEntity,
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

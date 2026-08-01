use serde::{Deserialize, Serialize};

use crate::vector::models::{
    academic::assignment::assignment_status::AssignmentStatus,
    date_time::{alarm::Alarm, date_time::DateTime},
    primitives::db_id::DatabaseId,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Milestone {
    pub id: DatabaseId,
    pub label: String,
    pub desc: Option<String>,
    pub status: AssignmentStatus,
    pub due_at: Option<DateTime>,
    pub alarms: Vec<Alarm>,
}

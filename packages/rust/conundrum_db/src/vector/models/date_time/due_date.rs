use serde::{Deserialize, Serialize};

use crate::vector::models::date_time::{alert::alert::Alert, date_time::DateTime};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DueDate {
    pub due_at: DateTime,
    pub alarms: Vec<Alert>,
}

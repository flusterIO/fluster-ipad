use serde::{Deserialize, Serialize};

use crate::vector::models::date_time::{alert::alert::Alert, date_time::DateTime};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Alarm {
    pub alert: Alert,
    pub time: DateTime,
}

use serde::{Deserialize, Serialize};

use crate::vector::models::date_time::alert::alert_severity::AlertSeverity;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Alert {
    title: String,
    body: String,
    severity: AlertSeverity,
}

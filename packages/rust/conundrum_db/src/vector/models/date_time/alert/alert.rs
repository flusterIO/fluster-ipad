use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

use crate::vector::models::date_time::alert::alert_severity::AlertSeverity;

#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub struct Alert {
    title: String,
    body: String,
    severity: AlertSeverity,
}

use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{date_time::alert::alert_severity::AlertSeverity, primitives::db_id::DatabaseId};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct Alert {
    pub id: DatabaseId,
    pub title: String,
    pub body: String,
    pub severity: AlertSeverity,
}

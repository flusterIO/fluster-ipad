use serde::{Deserialize, Serialize};

use crate::vector::models::date_time::alert::alert_severity::AlertSeverity;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Alert {
    TitleAndBody {
        body: String,
        title: String,
        severity: AlertSeverity,
    },
    /// The `Assigment` variant must be used attached to an assigment. Any other
    /// use will cause unexpected behavior. Who know's what'll happen...
    Assigment,
}

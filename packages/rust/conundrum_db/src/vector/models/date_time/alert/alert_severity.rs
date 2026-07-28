use serde::{Deserialize, Serialize};
use surrealdb::types::SurrealValue;

/// ## AlertSeverity
///
/// ```rs
/// Reminder = 0,
/// Assigment = 1,
/// Important = 2,
/// Urgent = 3,
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, SurrealValue)]
pub enum AlertSeverity {
    Reminder = 0,
    Assigment = 1,
    Important = 2,
    Urgent = 3,
}

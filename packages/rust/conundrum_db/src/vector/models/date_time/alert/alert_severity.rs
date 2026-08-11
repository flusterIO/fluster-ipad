use fake::Dummy;
use serde::{Deserialize, Serialize};

/// ## AlertSeverity
///
/// ```rs
/// Reminder = 0,
/// Assigment = 1,
/// Important = 2,
/// Urgent = 3,
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub enum AlertSeverity {
    Reminder = 0,
    Assigment = 1,
    Important = 2,
    Urgent = 3,
}

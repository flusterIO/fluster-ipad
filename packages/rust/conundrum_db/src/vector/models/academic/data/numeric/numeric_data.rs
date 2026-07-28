use serde::{Deserialize, Serialize};

use crate::vector::models::date_time::date_time::DateTime;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NumericData<T> {
    pub data: T,
    pub label: String,
    /// Describe your data for AI
    pub description: Option<String>,
    pub ctime: DateTime,
    pub utime: DateTime,
}

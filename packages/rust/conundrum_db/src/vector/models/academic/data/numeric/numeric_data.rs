use conundrum::lifted_models::primitives::date_time::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NumericData<T> {
    pub data: T,
    pub label: String,
    /// Describe your data for AI
    pub description: Option<String>,
    pub ctime: DateTime,
    pub utime: DateTime,
}

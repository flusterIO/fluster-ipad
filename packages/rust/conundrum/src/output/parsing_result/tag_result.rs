use serde::{Deserialize, Serialize};

#[derive(uniffi::Record, Debug, Serialize, Deserialize, Clone)]
pub struct TagResult {
    pub body: String,
}

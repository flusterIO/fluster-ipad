use serde::{Deserialize, Serialize};

#[derive(uniffi::Record, Debug, Serialize, Deserialize, Clone)]
pub struct TagResult {
    pub body: String,
}

impl TagResult {
    pub fn new(body: String) -> TagResult {
        TagResult { body }
    }
}

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Debug, Serialize, Deserialize, Clone)]
pub struct TagResult {
    pub body: String,
}

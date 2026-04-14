use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uniffi::Record;

#[typeshare]
#[derive(Serialize, Deserialize, Record)]
pub struct AIUserDetails {
    pub preferred_name: Option<String>,
}

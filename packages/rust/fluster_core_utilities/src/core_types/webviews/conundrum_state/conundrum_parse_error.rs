use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uniffi::Record;

#[typeshare]
#[derive(Record, Serialize, Deserialize, Clone)]
pub struct ConundrumParseErrorState {
    pub msg: String,
    pub code: u32,
}

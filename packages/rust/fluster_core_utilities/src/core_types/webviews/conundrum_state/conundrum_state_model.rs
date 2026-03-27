use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uniffi::Record;

use crate::core_types::webviews::conundrum_state::conundrum_parse_error::ConundrumParseErrorState;

#[typeshare]
#[derive(Record, Serialize, Deserialize, Clone)]
pub struct ConundrumState {
    pub error: Option<ConundrumParseErrorState>,
}

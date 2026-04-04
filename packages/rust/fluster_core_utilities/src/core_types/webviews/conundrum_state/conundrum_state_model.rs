use conundrum::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uniffi::Record;

#[typeshare]
#[derive(Record, Serialize, Deserialize, Clone)]
pub struct ConundrumState {
    pub error: Option<ConundrumErrorVariant>,
}

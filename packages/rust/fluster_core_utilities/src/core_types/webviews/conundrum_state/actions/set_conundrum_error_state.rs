use serde::Serialize;
use typeshare::typeshare;
use uniffi::Record;

use crate::core_types::webviews::conundrum_state::{
    conundrum_state_actions::ConundrumStateActions, conundrum_state_model::ConundrumState,
};

#[typeshare]
#[derive(Serialize, Record)]
pub struct SetConundrumErrorStateAction {
    pub r#type: ConundrumStateActions,
    pub payload: Option<ConundrumState>,
}

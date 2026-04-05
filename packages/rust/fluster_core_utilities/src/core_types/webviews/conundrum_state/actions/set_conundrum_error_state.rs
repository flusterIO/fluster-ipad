use conundrum::lang::runtime::state::conundrum_error::ConundrumError;
use serde::Serialize;
use typeshare::typeshare;
use uniffi::Record;

use crate::core_types::webviews::conundrum_state::conundrum_state_actions::ConundrumStateActions;

#[typeshare]
#[derive(Serialize, Record)]
pub struct SetConundrumErrorsStateAction {
    pub r#type: ConundrumStateActions,
    pub payload: Vec<ConundrumError>,
}

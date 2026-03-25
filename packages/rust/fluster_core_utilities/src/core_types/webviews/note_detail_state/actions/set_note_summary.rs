use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uniffi::Record;

use crate::core_types::webviews::note_detail_state::{
    actions::note_details_actions::NoteDetailActions, note_detail_state_model::SummaryState,
};

#[typeshare]
#[derive(Record, Serialize, Deserialize)]
pub struct SetNoteSummaryAction {
    pub r#type: NoteDetailActions,
    pub payload: SummaryState,
}

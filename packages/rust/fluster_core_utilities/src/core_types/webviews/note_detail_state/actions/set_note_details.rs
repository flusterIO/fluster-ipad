use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::note_detail_state::{
    actions::note_details_actions::NoteDetailActions, note_detail_state_model::NoteDetailState,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetNoteDetailsAction {
    pub r#type: NoteDetailActions,
    pub payload: Option<NoteDetailState>,
}

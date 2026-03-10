use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::editor_actions::EditorStateActions;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetAllCitationIdsPayload {
    all_citation_ids: Vec<String>,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetAllCitationIdsAction {
    pub r#type: EditorStateActions,
    pub payload: SetAllCitationIdsPayload,
}

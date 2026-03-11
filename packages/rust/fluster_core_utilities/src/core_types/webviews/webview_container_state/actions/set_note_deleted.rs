use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::webview_container_state::actions::webview_container_actions::WebviewContainerActions;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetNoteDeletedPayload {
    /// The id of the note the was deleted to be used for resetting state on the Typescript
    /// side if the note id matches the current state.
    pub note_id: String,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetNoteDeletedAction {
    pub r#type: WebviewContainerActions,
    pub payload: SetNoteDeletedPayload,
}

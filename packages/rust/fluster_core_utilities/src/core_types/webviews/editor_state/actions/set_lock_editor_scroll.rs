use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::editor_actions::EditorStateActions;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetLockEditorScrollToPreviewPayload {
    lock_editor_scroll_to_preview: bool,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetLockEditorScrollToPreviewAction {
    pub r#type: EditorStateActions,
    pub payload: SetLockEditorScrollToPreviewPayload,
}

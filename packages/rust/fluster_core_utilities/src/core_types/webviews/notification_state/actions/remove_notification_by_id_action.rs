use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::editor_actions::EditorStateActions;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct RemoveBannerNotificationByIdPayload {
    id: String,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct RemoveBannerNotificationByIdAction {
    pub r#type: EditorStateActions,
    pub payload: RemoveBannerNotificationByIdPayload,
}

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::{
    editor_state::editor_actions::EditorStateActions,
    notification_state::editor_banner_notification::EditorBannerNotification,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct AppendNotificationBannerPayload {
    notification: EditorBannerNotification,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct AppendNotificationBannerAction {
    pub r#type: EditorStateActions,
    pub payload: AppendNotificationBannerPayload,
}

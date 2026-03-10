use crate::core_types::webviews::notification_state::editor_banner_notification::EditorBannerNotification;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, Deserialize, Serialize)]
pub enum EditorLogSeverity {
    Info,
    Warn,
    Error,
    Fatal,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct EditorLog {
    pub msg: String,
    pub severity: EditorLogSeverity,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct NotificationState {
    pub logs: Vec<EditorLog>,
    pub banners: Vec<EditorBannerNotification>,
}

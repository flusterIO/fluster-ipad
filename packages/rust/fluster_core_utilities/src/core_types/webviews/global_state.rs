use crate::core_types::webviews::{
    ai_state::ai_state::AiState, editor_state::editor_state::EditorState,
    media_state::media_state::MediaState,
    notification_state::notification_state::NotificationState,
    webview_container_state::webview_container_state_model::WebviewContainerState,
};

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct GlobalWebviewState {
    pub container: WebviewContainerState,
    pub editor: EditorState,
    pub notifications: NotificationState,
    pub ai: AiState,
    pub media: MediaState,
}

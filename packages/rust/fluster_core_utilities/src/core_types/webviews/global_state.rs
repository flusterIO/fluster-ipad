use crate::core_types::webviews::{
    ai_state::ai_state_model::AiState, dictionary_state::dictionary_state::DictionaryState,
    editor_state::editor_state::EditorState, math_state::math_state_model::MathState,
    media_state::media_state::MediaState, note_detail_state::note_detail_state_model::NoteDetailState,
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
    pub note_details: Option<NoteDetailState>,
    pub dictionary: DictionaryState,
    pub math: MathState,
}

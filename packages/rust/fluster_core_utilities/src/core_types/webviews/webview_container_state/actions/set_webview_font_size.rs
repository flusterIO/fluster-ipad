use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::webview_container_state::{
    actions::webview_container_actions::WebviewContainerActions,
    webview_container_state_model::WebviewFontSize,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetWebviewFontSizePayload {
    /// The id of the note the was deleted to be used for resetting state on the Typescript
    /// side if the note id matches the current state.
    pub font_size: WebviewFontSize,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetWebviewFontSizeAction {
    pub r#type: WebviewContainerActions,
    pub payload: SetWebviewFontSizePayload,
}

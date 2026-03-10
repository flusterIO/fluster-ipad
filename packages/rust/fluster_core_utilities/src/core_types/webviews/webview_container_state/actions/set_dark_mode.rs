use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::webview_container_state::actions::webview_container_actions::WebviewContainerActions;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetDarkModePayload {
    dark_mode: bool,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetDarkModeAction {
    pub r#type: WebviewContainerActions,
    pub payload: SetDarkModePayload,
}

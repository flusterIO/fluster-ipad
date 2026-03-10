use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::webview_container_state::{
    actions::webview_container_actions::WebviewContainerActions, fluster_theme::FlusterTheme,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetFlusterThemePayload {
    fluster_theme: FlusterTheme,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetFlusterThemeAction {
    pub r#type: WebviewContainerActions,
    pub payload: SetFlusterThemePayload,
}

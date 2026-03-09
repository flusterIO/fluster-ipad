use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::{
    webview_environment::WebviewEnvironment,
    webviews::webview_container_state::{
        fluster_theme::FlusterTheme, webview_container_state_model::WebviewImplementation,
    },
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct WebviewContainerSharedInitialState {
    pub environment: WebviewEnvironment,
    pub dark_mode: bool,
    pub implementation: WebviewImplementation,
    pub fluster_theme: FlusterTheme,
}

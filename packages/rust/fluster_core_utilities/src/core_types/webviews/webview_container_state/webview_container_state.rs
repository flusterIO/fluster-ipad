use crate::core_types::webview_environment::WebviewEnvironment;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uniffi::{Enum, Record};

#[typeshare]
#[derive(Enum, strum_macros::Display, Serialize, Deserialize)]
pub enum SizableOption {
    #[serde(rename = "none")]
    #[strum(to_string = "none")]
    None,
    #[serde(rename = "small")]
    #[strum(to_string = "small")]
    Small,
    #[serde(rename = "smedium")]
    #[strum(to_string = "smedium")]
    Smedium,
    #[serde(rename = "medium")]
    #[strum(to_string = "medium")]
    Medium,
    #[serde(rename = "large")]
    #[strum(to_string = "large")]
    Large,
    #[serde(rename = "xl")]
    #[strum(to_string = "xl")]
    Xl,
    #[serde(rename = "xxl")]
    #[strum(to_string = "xxl")]
    Xxl,
    #[serde(rename = "fit")]
    #[strum(to_string = "fit")]
    Fit,
    #[serde(rename = "full")]
    #[strum(to_string = "full")]
    Full,
}

#[typeshare]
#[derive(Record, Serialize, Deserialize)]
pub struct WebviewContainerState {
    pub environment: Option<WebviewEnvironment>,
    pub size: SizableOption,
    pub wasm_loaded: bool,
}

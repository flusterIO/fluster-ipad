use crate::core_types::{
    webview_environment::WebviewEnvironment, webviews::webview_container_state::fluster_theme::FlusterTheme,
};
use conundrum::lang::lib::ui::shared_props::sizable_option::SizableOption;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use typeshare::typeshare;
use uniffi::{Enum, Record};

#[typeshare]
#[derive(Enum, Display, Serialize, Deserialize)]
pub enum WebviewImplementation {
    #[serde(rename = "bib-editor")]
    #[strum(to_string = "bib-editor")]
    BibEditor,
    #[serde(rename = "mdx-editor")]
    #[strum(to_string = "mdx-editor")]
    MdxEditor,
    #[serde(rename = "mdx-viewer")]
    #[strum(to_string = "mdx-viewer")]
    MdxViewer,
    #[serde(rename = "dictionary")]
    #[strum(to_string = "dictionary")]
    Dictionary,
    #[serde(rename = "note-details")]
    #[strum(to_string = "note-details")]
    NoteDetails,
    #[serde(rename = "development")]
    #[strum(to_string = "development")]
    Development,
    #[serde(rename = "pending")]
    #[strum(to_string = "pending")]
    AwaitingData,
}

#[typeshare]
#[derive(Enum, Display, Serialize, Deserialize)]
pub enum WebviewFontSize {
    #[serde(rename = "small")]
    #[strum(to_string = "small")]
    Small,
    #[serde(rename = "base")]
    #[strum(to_string = "base")]
    Base,
    #[serde(rename = "large")]
    #[strum(to_string = "large")]
    Large,
    #[serde(rename = "xl")]
    #[strum(to_string = "xl")]
    Xl,
    #[serde(rename = "xxl")]
    #[strum(to_string = "xxl")]
    Xxl,
}

#[typeshare]
#[derive(Record, Serialize, Deserialize)]
pub struct WebviewContainerState {
    pub environment: Option<WebviewEnvironment>,
    pub size: SizableOption,
    pub wasm_loaded: bool,
    pub dark_mode: bool,
    pub implementation: WebviewImplementation,
    pub fluster_theme: FlusterTheme,
    pub font_size: WebviewFontSize,
}

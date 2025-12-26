use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum DictionaryWebviewStorageKeys {
    #[serde(rename = "dict-scroll-pos-portrait")]
    #[strum(to_string = "dict-scroll-pos-portrait")]
    ScrollPositionPortrait,
    #[serde(rename = "dict-scroll-pos-landscape")]
    #[strum(to_string = "dict-scroll-pos-landscape")]
    ScrollPositionLandscape,
    #[serde(rename = "dict-data")]
    #[strum(to_string = "dict-data")]
    DictionaryData,
}

/// From typescript to swift.
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum DictionaryWebviewActions {
    #[serde(rename = "request-dictionary-data")]
    #[strum(to_string = "request-dictionary-data")]
    RequestDictionaryData,
    #[serde(rename = "set-webview-loaded")]
    #[strum(to_string = "set-webview-loaded")]
    SetWebviewLoaded,
}

/// From swift to typescript
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum DictionaryWebviewEvents {
    #[serde(rename = "set-initial-color-scheme")]
    #[strum(to_string = "set-initial-color-scheme")]
    SetInitialColorScheme,
    #[serde(rename = "set-dictionary-data")]
    #[strum(to_string = "set-dictionary-data")]
    SetDictionaryData,
    #[serde(rename = "set-code-theme")]
    #[strum(to_string = "set-code-theme")]
    SetCodeTheme,
}

#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum DictionaryWebviewIds {
    #[serde(rename = "dictionary-container")]
    #[strum(to_string = "dictionary-container")]
    DictionaryContainer,
}

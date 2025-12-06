use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// From typescript to swift.
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum BibtexEditorWebviewActions {
    #[serde(rename = "request-note-detail-data")]
    #[strum(to_string = "request-note-detail-data")]
    RequestBibtexEditorData,
    #[serde(rename = "set-note-detail-webview-loaded")]
    #[strum(to_string = "set-note-detail-webview-loaded")]
    SetWebviewLoaded,
}

/// From swift to typescript
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum BibtexEditorWebviewEvents {
    #[serde(rename = "set-initial-color-scheme")]
    #[strum(to_string = "set-initial-color-scheme")]
    SetInitialColorScheme,
    #[serde(rename = "set-note-details")]
    #[strum(to_string = "set-note-details")]
    SetBibtexEditorContent,
    #[serde(rename = "set-editor-keymap")]
    #[strum(to_string = "set-editor-keymap")]
    SetEditorKeymap,
    #[serde(rename = "set-code-theme")]
    #[strum(to_string = "set-code-theme")]
    SetCodeTheme,
    #[serde(rename = "set-code-theme-light")]
    #[strum(to_string = "set-code-theme-light")]
    SetCodeThemeLight,
    #[serde(rename = "set-code-theme-dark")]
    #[strum(to_string = "set-code-theme-dark")]
    SetCodeThemeDark,
}

#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum BibtexEditorWebviewLocalStorageKeys {
    #[serde(rename = "bibtex-editor-initial-value")]
    #[strum(to_string = "bibtex-editor-initial-value")]
    InitialValue,
}

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// From typescript to swift.
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum BibtexEditorWebviewActions {
    #[serde(rename = "request-bibtex-data")]
    #[strum(to_string = "request-bibtex-data")]
    RequestBibtexEditorData,
    #[serde(rename = "set-bibtex-webview-loaded")]
    #[strum(to_string = "set-bibtex-webview-loaded")]
    SetWebviewLoaded,
    #[serde(rename = "on-bib-editor-change")]
    #[strum(to_string = "on-bib-editor-change")]
    OnEditorChange,
}

/// From swift to typescript
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum BibtexEditorWebviewEvents {
    #[serde(rename = "set-bib-initial-color-scheme")]
    #[strum(to_string = "set-bib-initial-color-scheme")]
    SetInitialColorScheme,
    #[serde(rename = "set-bibtex-data")]
    #[strum(to_string = "set-bibtex-data")]
    SetBibtexEditorContent,
    #[serde(rename = "set-bib-editor-keymap")]
    #[strum(to_string = "set-bib-editor-keymap")]
    SetEditorKeymap,
    #[serde(rename = "set-bib-code-theme")]
    #[strum(to_string = "set-bib-code-theme")]
    SetCodeTheme,
    #[serde(rename = "set-bib-code-theme-light")]
    #[strum(to_string = "set-bib-code-theme-light")]
    SetCodeThemeLight,
    #[serde(rename = "set-bib-code-theme-dark")]
    #[strum(to_string = "set-bib-code-theme-dark")]
    SetCodeThemeDark,
}

#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum BibtexEditorWebviewLocalStorageKeys {
    #[serde(rename = "bibtex-editor-initial-value")]
    #[strum(to_string = "bibtex-editor-initial-value")]
    InitialValue,
}

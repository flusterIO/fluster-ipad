use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// From typescript to swift.
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum SplitviewEditorWebviewActions {
    #[serde(rename = "request-editor-data")]
    #[strum(to_string = "request-editor-data")]
    RequestSplitviewEditorData,
    #[serde(rename = "request-parsed-mdx")]
    #[strum(to_string = "request-parsed-mdx")]
    RequestParsedMdxContent,
    #[serde(rename = "on-editor-change")]
    #[strum(to_string = "on-editor-change")]
    OnEditorChange,
    #[serde(rename = "set-editor-webview-loaded")]
    #[strum(to_string = "set-editor-webview-loaded")]
    SetWebviewLoaded,
    #[serde(rename = "tag-click-event")]
    #[strum(to_string = "tag-click-event")]
    OnTagClick,
    #[serde(rename = "set-is-landscape-view")]
    #[strum(to_string = "set-is-landscape-view")]
    SetIsLandscape,
}

/// From swift to typescript
#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum SplitviewEditorWebviewEvents {
    #[serde(rename = "set-initial-color-scheme")]
    #[strum(to_string = "set-initial-color-scheme")]
    SetInitialColorScheme,
    #[serde(rename = "set-editor-content")]
    #[strum(to_string = "set-editor-content")]
    SetSplitviewEditorContent,
    #[serde(rename = "set-parsed-mdx-content")]
    #[strum(to_string = "set-parsed-mdx-content")]
    SetParsedMdxContent,
    #[serde(rename = "set-editor-content-str")]
    #[strum(to_string = "set-editor-content-str")]
    SetParsedMdxContentString,
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
    #[serde(rename = "reset-mdx-preview-scroll-position")]
    #[strum(to_string = "reset-mdx-preview-scroll-position")]
    ResetPreviewScrollPosition,
    #[serde(rename = "mdx-parsing-error")]
    #[strum(to_string = "mdx-parsing-error")]
    EmitMdxParsingError,
    #[serde(rename = "mdx-parsing-success")]
    #[strum(to_string = "mdx-parsing-success")]
    EmitMdxParsingSuccess,
}

#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum SplitviewEditorWebviewLocalStorageKeys {
    #[serde(rename = "editor-initial-value")]
    #[strum(to_string = "editor-initial-value")]
    InitialValue,
    #[serde(rename = "parsed-mdx-data")]
    #[strum(to_string = "parsed-mdx-data")]
    ParsedMdxData,
    #[serde(rename = "editor-code-theme")]
    #[strum(to_string = "editor-code-theme")]
    CodeTheme,
    #[serde(rename = "editor-code-theme-dark")]
    #[strum(to_string = "editor-code-theme-dark")]
    CodeThemeDark,
    #[serde(rename = "editor-code-theme-light")]
    #[strum(to_string = "editor-code-theme-light")]
    CodeThemeLight,
    #[serde(rename = "editor-keymap")]
    #[strum(to_string = "editor-keymap")]
    EditorKeymap,
    #[serde(rename = "splitview-editor-scroll-position-portrait")]
    #[strum(to_string = "splitview-editor-scroll-position-portrait")]
    ScrollPositionPortrait,
    #[serde(rename = "splitview-editor-scroll-position-landscape")]
    #[strum(to_string = "splitview-editor-scroll-position-landscape")]
    ScrollPositionLandscape,
}

#[typeshare]
#[derive(strum_macros::Display, Serialize, Deserialize)]
pub enum SplitviewEditorWebviewIds {
    #[serde(rename = "mdx-preview-portrait")]
    #[strum(to_string = "mdx-preview-portrait")]
    PortraitPreview,
    #[serde(rename = "mdx-preview-landscape")]
    #[strum(to_string = "mdx-preview-landscape")]
    LandscapePreview,
}

use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uniffi::{Enum, Record};

use crate::core_types::webviews::{
    editor_save_method::EditorSaveMethod,
    editor_state::{editor_theme::CodeEditorTheme, snippet_state::SnippetState},
};

#[typeshare]
#[derive(Record, Serialize, Deserialize)]
/// Basically a Partial<BibEntryModel> that's cross language, to be sent to the
/// editor.
pub struct EditorCitation {
    pub key: String,
    pub html: String,
}

#[typeshare]
#[derive(Record, Serialize, Deserialize)]
/// Basically a Partial<TagModel> that's cross language, to be sent to the
/// editor.
pub struct EditorTag {
    pub body: String,
}

#[typeshare]
#[derive(Enum, Serialize, Deserialize)]
pub enum EditorView {
    Pending,
    Splitview,
    PreviewOnly,
}

#[typeshare]
#[derive(Enum, strum_macros::Display, Serialize, Deserialize)]
pub enum CodeEditorBaseKeymap {
    #[serde(rename = "default")]
    #[strum(to_string = "default")]
    Default,
    #[serde(rename = "vsCode")]
    #[strum(to_string = "vsCode")]
    VsCode,
}

#[typeshare]
#[derive(Record, Serialize, Deserialize)]
pub struct EditorState {
    pub note_id: Option<String>,
    #[serde(rename = "baseKeymap")]
    pub base_keymap: CodeEditorBaseKeymap,
    pub citations: Vec<EditorCitation>,
    pub theme: CodeEditorTheme,
    pub tags: Vec<EditorTag>,
    #[serde(rename = "allCitationIds")]
    pub all_citation_ids: Vec<String>,
    pub value: String,
    #[serde(rename = "parsedValue")]
    pub parsed_value: Option<String>,
    #[serde(rename = "haveSetInitialValue")]
    pub have_set_initial_value: bool,
    #[serde(rename = "editorView")]
    pub editor_view: EditorView,
    #[serde(rename = "snippetProps")]
    pub snippet_props: SnippetState,
    #[serde(rename = "lockEditorScrollToPreview")]
    pub lock_editor_scroll_to_preview: bool,
    #[serde(rename = "saveMethod")]
    pub save_method: EditorSaveMethod,
}

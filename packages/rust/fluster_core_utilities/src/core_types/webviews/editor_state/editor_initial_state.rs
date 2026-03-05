use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::{
    editor_save_method::EditorSaveMethod,
    editor_state::{
        editor_keymap::CodeEditorKeymap, editor_theme::CodeEditorTheme, snippet_state::SnippetState,
    },
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct EditorInitialStatePayload {
    pub note_id: String,
    pub keymap: CodeEditorKeymap,
    pub theme: CodeEditorTheme,
    #[serde(rename = "allCitationIds")]
    pub all_citation_ids: Vec<String>,
    pub value: String,
    #[serde(rename = "parsedValue")]
    pub parsed_value: String,
    #[serde(rename = "haveSetInitialValue")]
    pub have_set_initial_value: bool,
    #[serde(rename = "snippetProps")]
    pub snippet_props: SnippetState,
    #[serde(rename = "lockEditorScrollToPreview")]
    pub lock_editor_scroll_to_preview: bool,
    #[serde(rename = "saveMethod")]
    pub save_method: EditorSaveMethod,
}

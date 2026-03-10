use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::editor_state::{
    editor_actions::EditorStateActions, snippet_state::SnippetState,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetSnippetPropsPayload {
    #[serde(rename = "snippetProps")]
    snippet_props: SnippetState,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetSnippetPropsAction {
    pub r#type: EditorStateActions,
    pub payload: SetSnippetPropsPayload,
}

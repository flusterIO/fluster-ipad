use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Clone, Serialize, Deserialize)]
pub struct BibtexEditorState {
    pub value: String,
}

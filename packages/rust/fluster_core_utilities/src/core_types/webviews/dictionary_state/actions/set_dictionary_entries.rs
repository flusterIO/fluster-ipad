use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::core_types::webviews::dictionary_state::{
    actions::dictionary_state_actions::DictionaryStateActions, dictionary_state::DictionaryState,
};

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct SetDictionaryEntriesAction {
    pub r#type: DictionaryStateActions,
    pub payload: DictionaryState,
}

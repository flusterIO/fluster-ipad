use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use crate::{
    lang::{
        lib::ui::ui_types::children::Children,
}
};
/// Both the label and body fields are ***un-compiled*** Conundrum content.
#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug, uniffi::Record)]
pub struct DictionaryEntryResult {
    pub label: String,
    pub body: String,
}


/// Both the label and body fields are ***un-compiled*** Conundrum content.
#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DictionaryEntryResultUnCompiled {
    pub label: Children,
    /// Used for unique-ness comparison.
    pub label_string: String,
    pub body: Children,
}





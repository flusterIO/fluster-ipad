use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use typeshare::typeshare;

#[typeshare]
#[derive(Display, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutoInsertedComponentName {
    #[serde(rename = "NoteLink")]
    #[strum(to_string = "NoteLink")]
    NoteLink,
    #[serde(rename = "AutoInsertedTag")]
    #[strum(to_string = "AutoInsertedTag")]
    AutoInsertedTag,
    #[serde(rename = "FlusterCitation")]
    #[strum(to_string = "FlusterCitation")]
    FlusterCitation,
    #[serde(rename = "DictionaryEntry")]
    #[strum(to_string = "DictionaryEntry")]
    DictionaryEntry,
}

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, Serialize, Deserialize, PartialEq, strum_macros::Display)]
pub enum ParserId {
    #[serde(rename = "tags")]
    #[strum(to_string = "tags")]
    Tags,
    #[serde(rename = "citations")]
    #[strum(to_string = "citations")]
    Citations,
    #[serde(rename = "dictionary")]
    #[strum(to_string = "dictionary")]
    Dictionary,
    #[serde(rename = "note_link")]
    #[strum(to_string = "note_link")]
    NoteLink,
    #[serde(rename = "docs")]
    #[strum(to_string = "docs")]
    Documentation,
    #[serde(rename = "ai")]
    #[strum(to_string = "ai")]
    AiTrigger,
}

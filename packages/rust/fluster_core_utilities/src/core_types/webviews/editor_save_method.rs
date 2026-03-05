use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, strum_macros::Display, Serialize, Deserialize)]
pub enum EditorSaveMethod {
    #[serde(rename = "on-save")]
    #[strum(to_string = "on-save")]
    OnSave,
    #[serde(rename = "on-change")]
    #[strum(to_string = "on-change")]
    OnChange,
}

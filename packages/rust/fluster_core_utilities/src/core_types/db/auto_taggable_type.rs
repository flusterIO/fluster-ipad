use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(uniffi::Enum, strum_macros::Display, Deserialize, Serialize)]
pub enum AutoTaggableType {
    #[serde(rename = "tag")]
    #[strum(to_string = "tag")]
    Tag,
    #[serde(rename = "topic")]
    #[strum(to_string = "topic")]
    Topic,
    #[serde(rename = "subject")]
    #[strum(to_string = "subject")]
    Subject,
}

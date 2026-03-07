use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(uniffi::Record, Debug, Serialize, Deserialize, Clone)]
pub struct CitationResult {
    /// The parsed citation key.
    pub(crate) citation_key: String,
    /// The complete bibtex entry
    pub(crate) idx: u8,
}

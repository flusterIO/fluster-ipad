use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

// TODO: Remove typehare here and move references to the wasm-bindgen generated code.
#[wasm_bindgen]
#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct CodeBlockParsingResult {
    pub(crate) full_match: String,
    pub(crate) language_tag: String,
    pub(crate) block_content: String,
    pub(crate) meta_data: Option<String>,
}

#[wasm_bindgen]
impl CodeBlockParsingResult {
    #[wasm_bindgen(constructor)]
    pub fn new(
        full_match: String,
        language_tag: String,
        block_content: String,
        meta_data: Option<String>,
    ) -> Self {
        CodeBlockParsingResult {
            full_match,
            language_tag,
            block_content,
            meta_data,
        }
    }
    pub fn get_block_content_rust(&self) -> String {
        self.block_content.clone()
    }
    pub fn set_block_content_rust(&mut self, block_content: String) {
        self.block_content = block_content;
    }
    pub fn get_full_match_rust(&self) -> String {
        self.full_match.clone()
    }
    pub fn get_lang_rust(self) -> String {
        self.language_tag
    }
    #[wasm_bindgen(getter)]
    pub fn full_match(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.full_match).unwrap_or(JsValue::undefined())
    }
    #[wasm_bindgen(setter)]
    pub fn set_full_match(&mut self, full_match: String) {
        self.full_match = full_match;
    }
    #[wasm_bindgen(getter)]
    pub fn language_tag(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.language_tag).unwrap_or(JsValue::undefined())
    }
    #[wasm_bindgen(setter)]
    pub fn set_language_tag(&mut self, language_tag: String) {
        self.language_tag = language_tag;
    }
    #[wasm_bindgen(getter)]
    pub fn block_content(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.block_content).unwrap_or(JsValue::undefined())
    }
    #[wasm_bindgen(setter)]
    pub fn set_block_content(&mut self, block_content: String) {
        self.block_content = block_content;
    }
    #[wasm_bindgen(getter)]
    pub fn meta_data_rust(&self) -> Option<String> {
        self.meta_data.clone()
    }
}

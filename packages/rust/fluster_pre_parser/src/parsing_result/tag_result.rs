use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen]
#[derive(uniffi::Record, Debug, Serialize, Deserialize, Clone)]
pub struct TagResult {
    pub(crate) body: String,
}

#[wasm_bindgen]
impl TagResult {
    #[wasm_bindgen(constructor)]
    pub fn new(body: String) -> TagResult {
        TagResult { body }
    }
    #[wasm_bindgen(getter)]
    pub fn body(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.body).unwrap_or(JsValue::null())
    }
    #[wasm_bindgen(setter)]
    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }
}

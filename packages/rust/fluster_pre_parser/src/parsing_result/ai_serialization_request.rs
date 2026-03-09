use fluster_core_utilities::core_types::ai::code_block_parsing_result::CodeBlockParsingResult;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[derive(uniffi::Enum, Serialize, Deserialize, Debug, Clone)]
pub enum AiSerializationRequestType {
    CreateNoteSpecificStudyGuide,
    SummarizeNote,
    RecommendResearch,
}

#[wasm_bindgen]
#[derive(uniffi::Record, Debug, Serialize, Deserialize, Clone)]
pub struct AiSerializationRequestPhase1 {
    pub(crate) parsing_result: CodeBlockParsingResult,
}

#[wasm_bindgen]
impl AiSerializationRequestPhase1 {
    #[wasm_bindgen]
    pub fn parsing_result(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.parsing_result).unwrap_or(JsValue::undefined())
    }
    #[wasm_bindgen]
    pub fn set_parsing_result(&mut self, parsing_result: CodeBlockParsingResult) {
        self.parsing_result = parsing_result;
    }
}

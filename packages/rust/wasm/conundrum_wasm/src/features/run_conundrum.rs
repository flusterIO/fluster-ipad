// use conundrum::lang::runtime::run_conundrum::{self, run_conundrum};
// use conundrum::lang::runtime::state::parse_state::ConundrumCompileTarget;
// use wasm_bindgen::JsValue;
// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn compile_cdrm(cdrm: String, ui_params: JsValue, trusted: bool) -> Result<JsValue, JsValue> {
//     let res = run_conundrum(run_conundrum::ParseConundrumOptions { note_id: None, content: cdrm, modifiers: vec![], hide_components: vec![], ui_params: serde_wasm_bindgen::from_value(ui_params)?, target: ConundrumCompileTarget::Html, trusted })
//         .map_err(|x| {
//                                                   serde_wasm_bindgen::to_value(&x).unwrap_or_default()
//         })?;

//     Ok(serde_wasm_bindgen::to_value(&res)?)
// }

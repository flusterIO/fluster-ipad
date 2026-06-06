use conundrum::lang::{
    lib::general::pagination::pagination_params::PaginationParams,
    runtime::queries::emojis::search_emojis_to_docs_container,
};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn search_conundrum_emojis_in_docs_container(query: String,
                                                 container_id: String,
                                                 page: usize,
                                                 per_page: usize)
                                                 -> Result<JsValue, JsValue> {
    let res = search_emojis_to_docs_container(query,
                                              Some(PaginationParams { per_page: per_page as u32,
                                                                      page: page as u32 }))
        .map_err(|x| {
                                                  serde_wasm_bindgen::to_value(&x).unwrap_or_default()
        })?;

    Ok(serde_wasm_bindgen::to_value(&res)?)
}

use conundrum::lang::{
    lib::general::pagination::pagination_params::PaginationParams, runtime::queries::emojis::search_emojis,
};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen]
pub fn search_conundrum_emojis_and_append_to_container(query: String,
                                                       container_id: String,
                                                       page: usize,
                                                       per_page: usize)
                                                       -> Result<JsValue, JsValue> {
    let res = search_emojis(query,
                            Some(PaginationParams { per_page: per_page as u32,
                                                    page: page as u32 }));
    Ok(serde_wasm_bindgen::to_value(&res)?)
}

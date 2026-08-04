use conundrum::lang::{
    lib::general::pagination::pagination_params::PaginationParams, runtime::queries::emojis::search_emojis,
};
use wasm_bindgen::prelude::*;
// pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn search_conundrum_emojis(query: String, page: usize, per_page: usize) -> Result<JsValue, JsValue> {
    let res = search_emojis(query,
                            Some(PaginationParams { per_page: per_page as u32,
                                                    page: page as u32 }));
    Ok(serde_wasm_bindgen::to_value(&res)?)
}

// #[wasm_bindgen]
// pub async fn pre_parse_mdx(opts: ParseMdxOptions) -> MdxParsingResult {
//     fluster_pre_parser::parse::by_regex::parse_mdx_by_regex::parse_mdx_string_to_mdx_result(&opts)
//         .await
// }

use conundrum::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;
use conundrum::lang::{
    lib::general::pagination::pagination_params::PaginationParams,
    runtime::queries::emojis::search_emojis_to_docs_container,
};
use gloo::console;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

/// Not working at the moment. It's currently unable to find the element despite it being in the
/// DOM.
// #[wasm_bindgen]
// pub fn search_conundrum_emojis_and_append_to_container(query: String,
//                                                        container_id: String,
//                                                        page: usize,
//                                                        per_page: usize)
//                                                        -> Result<(), JsValue> {
//     console::info!("Looking for container with an id of `{:?}`", container_id.clone());
//     let res = search_emojis_to_docs_container(query,
//                                               Some(PaginationParams { per_page: per_page as u32,
//                                                                       page: page as u32 }))
//         .map_err(|x| {
//                                                   serde_wasm_bindgen::to_value(&x).unwrap_or_default()
//         })?;

//     let doc = web_sys::Document::new().unwrap();

//     console::info!("{:#?}", doc);

//     if let Some(em) = web_sys::Document::new().map_err(|e| {
//                                                   console::error!("Error: {:?}", e.clone());
//                                                   let err = ConundrumErrorVariant::WasmError(format!("{:?}", e));
//                                                   serde_wasm_bindgen::to_value(&err).unwrap_or_else(|_| e.clone())
//                                               })?
//                                               .get_element_by_id(container_id.as_str())
//     {
//         console::info!("Setting emoji children...");
//         em.set_inner_html(res.names.iter().map(|item| item.html.clone()).collect::<Vec<String>>().join("\n").as_str());
//     } else {
//         console::warn!("Unable to append children as the parent emoji container could not be found.")
//     }
//     Ok(())
// }


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

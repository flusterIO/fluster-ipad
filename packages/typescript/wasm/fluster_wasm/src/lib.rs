use fluster_pre_parser::{
    self, parse::by_regex::parse_mdx_by_regex::ParseMdxOptions,
    parsing_result::mdx_parsing_result::MdxParsingResult,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello from Fluster wasm {name}!!!"));
}

#[wasm_bindgen]
pub async fn pre_parse_mdx(opts: ParseMdxOptions) -> MdxParsingResult {
    fluster_pre_parser::parse::by_regex::parse_mdx_by_regex::parse_mdx_string_to_mdx_result(&opts)
        .await
}

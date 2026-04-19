use crate::{
    lang::runtime::state::parse_state::{ConundrumModifier, ParseState},
    output::html::glue::{
        component_glue_manager::{JS_GLUE_CODE_MAP, WebGlueCodeGeneralFiles},
        glue_asset::GlueCssAsset,
        individual_glue_assets::katex_css::KatexCssAsset,
    },
};

#[derive(Debug)]
pub struct WebGlueAssetData {
    pub js: String,
    pub css: String,
}

pub fn get_glue_asset_data(state: &ParseState, standalone: &bool) -> WebGlueAssetData {
    let mut js_string = String::from("");
    // Get all the css that goes into the app regardless.
    let mut css_string = match standalone {
        true => JS_GLUE_CODE_MAP.get(WebGlueCodeGeneralFiles::Styles.to_string().as_str())
                                .expect("Css better be there...")
                                .to_string(),
        false => String::from(""),
    };

    let is_standalone = state.contains_modifier(&ConundrumModifier::Standalone);
    KatexCssAsset::default().append_to_css(&mut css_string, &is_standalone);

    // Get all of the conditional css.
    // Get general javascript glue code.
    // Get component glue code
    for c in &state.data.get_included_components() {
        if let Some(js) = JS_GLUE_CODE_MAP.get(c) {
            js_string += js.to_string().as_str();
        }
    }

    WebGlueAssetData { css: css_string,
                       js: js_string }
}

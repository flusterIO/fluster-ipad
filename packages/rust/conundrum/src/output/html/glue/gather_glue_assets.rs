use crate::{
    lang::runtime::traits::conundrum_input::ArcState,
    output::html::glue::{
        component_glue_manager::JS_GLUE_CODE_MAP,
        glue_asset::GlueCssAsset,
        individual_glue_assets::{
            katex_ams_regular::KatexFontAssetAmsRegular, katex_caligraphic_bold::KatexFontAssetCaligraphicBold,
            katex_caligraphic_regular::KatexFontAssetCaligraphicRegular, katex_css::KatexCssAsset,
            katex_fraktur_bold::KatexFontAssetFrakturBold, katex_fraktur_regular::KatexFontAssetFrakturRegular,
            katex_main_bold::KatexFontAssetMainBold, katex_main_italic::KatexFontAssetMainItalic,
            katex_main_regular::KatexFontAssetMainRegular, katex_math_bolditalic::KatexFontAssetMathBolditalic,
            katex_math_italic::KatexFontAssetMathItalic, katex_sansserif_bold::KatexFontAssetSansserifBold,
            katex_sansserif_italic::KatexFontAssetSansserifItalic,
            katex_sansserif_regular::KatexFontAssetSansserifRegular, katex_script_regular::KatexFontAssetScriptRegular,
            katex_size1_regular::KatexFontAssetSize1Regular, katex_size2_regular::KatexFontAssetSize2Regular,
            katex_size3_regular::KatexFontAssetSize3Regular, katex_size4_regular::KatexFontAssetSize4Regular,
            katex_typewriter_regular::KatexFontAssetTypewriterRegular, lucide_font::LucideFontAsset,
            nerd_font_asset::NerdFontAsset,
        },
        webglue_general_files::WebGlueCodeGeneralFiles,
    },
};

#[derive(Debug)]
pub struct WebGlueAssetData {
    pub js: String,
    pub css: String,
}

pub fn get_glue_asset_data(_state: ArcState) -> WebGlueAssetData {
    let mut js_string = String::from("");
    let state = _state.read_arc();
    let is_standalone = state.is_standalone();
    // Get all the css that goes into the app regardless.
    let mut css_string = match is_standalone {
        true => {
            if let Some(g) = JS_GLUE_CODE_MAP.get(WebGlueCodeGeneralFiles::Styles.to_string().as_str()) {
                if let Ok(s) = String::from_utf8(g.to_vec()) {
                    s
                } else {
                    String::from("")
                }
            } else {
                String::from("")
            }
        }
        false => String::from(""),
    };

    KatexCssAsset::default().append_to_css(&mut css_string, &is_standalone);

    if is_standalone {
        KatexFontAssetAmsRegular::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetCaligraphicBold::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetCaligraphicRegular::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetFrakturBold::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetFrakturRegular::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetMainItalic::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetMainBold::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetMainItalic::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetMainRegular::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetMathBolditalic::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetMathItalic::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetSansserifBold::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetSansserifItalic::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetSansserifRegular::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetScriptRegular::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetSize1Regular::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetSize2Regular::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetSize3Regular::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetSize4Regular::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontAssetTypewriterRegular::default().append_to_css(&mut css_string, &is_standalone);
        NerdFontAsset::default().append_to_css(&mut css_string, &is_standalone);
        LucideFontAsset {}.append_to_css(&mut css_string, &is_standalone);
    }

    // Get all of the conditional css.
    // Get general javascript glue code.
    // Get component glue code
    for c in &state.data.get_included_components() {
        if let Some(js_bytes) = JS_GLUE_CODE_MAP.get(c) {
            if let Ok(js) = String::from_utf8(js_bytes.to_vec()) {
                js_string += js.as_str();
            }
        }
    }

    WebGlueAssetData { css: css_string,
                       js: js_string }
}

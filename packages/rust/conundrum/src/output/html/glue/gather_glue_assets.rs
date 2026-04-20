use crate::{
    lang::runtime::traits::conundrum_input::ArcState,
    output::html::glue::{
        component_glue_manager::JS_GLUE_CODE_MAP,
        glue_asset::GlueCssAsset,
        individual_glue_assets::{
            katex_ams_regular::KatexFontAmsReg, katex_caligraphic_bold::KatexFontCaligBold,
            katex_caligraphic_regular::KatexFontCaligReg, katex_css::KatexCssAsset,
            katex_fraktur_bold::KatexFontFrakturBold, katex_fraktur_regular::KatexFontFracturReg,
            katex_main_bold::KatexFontMainBold, katex_main_italic::KatexFontMainItalic,
            katex_main_regular::KatexFontMainReg, katex_math_bolditalic::KatexFontMathBoldItalic,
            katex_math_italic::KatexFontMathItalic, katex_sansserif_bold::KatexFontSansBold,
            katex_sansserif_italic::KatexFontSansItalic, katex_sansserif_regular::KatexFontSansReg,
            katex_script_regular::KatexFontScriptReg, katex_size1_regular::KatexFontSize1,
            katex_size2_regular::KatexFontSize2, katex_size3_regular::KatexFontSize3,
            katex_size4_regular::KatexFontSize4, katex_typewriter_regular::KatexFontTypewriterReg,
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
        KatexFontAmsReg::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontCaligBold::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontCaligReg::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontFrakturBold::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontFracturReg::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontMainItalic::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontMainBold::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontMainItalic::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontMainReg::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontMathBoldItalic::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontMathItalic::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontSansBold::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontSansItalic::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontSansReg::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontScriptReg::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontSize1::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontSize2::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontSize3::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontSize4::default().append_to_css(&mut css_string, &is_standalone);
        KatexFontTypewriterReg::default().append_to_css(&mut css_string, &is_standalone);
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

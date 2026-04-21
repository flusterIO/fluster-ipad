use crate::output::{
    general::component_constants::{
        auto_inserted_component_name::AutoInsertedComponentName, component_ids::EmbeddableComponentId,
    },
    html::glue::webglue_general_files::WebGlueCodeGeneralFiles,
};
use dashmap::DashMap;
use lazy_static::lazy_static;

#[derive(Clone)]
pub enum AnyGlueCodeKey {
    AutoInserted(AutoInsertedComponentName),
    Embeddable(EmbeddableComponentId),
    General(WebGlueCodeGeneralFiles),
}

lazy_static! {
    pub static ref JS_GLUE_CODE_MAP: DashMap<String, Vec<u8>> = {
        let map: DashMap<String, Vec<u8>> = DashMap::new();
        map.insert(WebGlueCodeGeneralFiles::Styles.to_string(),
                   include_str!("../standalone/conundrum.css").to_string().as_bytes().to_vec());
        map.insert(WebGlueCodeGeneralFiles::NerdFont.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/Fira_Code_Regular.ttf").to_vec());
        map.insert(WebGlueCodeGeneralFiles::KatexCss.to_string(),
                   include_str!("../glue/component_glue/other/katex.min.css").to_string().as_bytes().to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_ams_regular.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_AMS-Regular.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_caligraphic_bold.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Caligraphic-Bold.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_caligraphic_regular.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Caligraphic-Regular.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_fraktur_bold.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Fraktur-Bold.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_fraktur_regular.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Fraktur-Regular.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_main_bold.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Main-Bold.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_main_bolditalic.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Main-BoldItalic.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_main_italic.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Main-Italic.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_main_regular.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Main-Regular.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_math_bolditalic.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Math-BoldItalic.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_math_italic.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Math-Italic.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_sansserif_bold.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_SansSerif-Bold.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_sansserif_italic.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_SansSerif-Italic.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_sansserif_regular.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_SansSerif-Regular.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_script_regular.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Script-Regular.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_size1_regular.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Size1-Regular.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_size2_regular.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Size2-Regular.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_size3_regular.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Size3-Regular.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_size4_regular.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Size4-Regular.woff2").to_vec());
        map.insert(WebGlueCodeGeneralFiles::Katex_typewriter_regular.to_string(),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Typewriter-Regular.woff2").to_vec());

        // --- Components ---
        map.insert(AutoInsertedComponentName::AutoInsertedCodeBlock.to_string(),
                   include_str!("./component_glue_output/code_block.js").to_string().as_bytes().to_vec());
        map.insert(AutoInsertedComponentName::NoteLink.to_string(),
                   include_str!("./component_glue_output/note_link.js").to_string().as_bytes().to_vec());
        map
    };
}

impl JS_GLUE_CODE_MAP {
    pub fn get_string_by_key(k: AnyGlueCodeKey) -> Option<String> {
        let key = match k {
            AnyGlueCodeKey::AutoInserted(n) => n.to_string(),
            AnyGlueCodeKey::Embeddable(n) => n.to_string(),
            AnyGlueCodeKey::General(n) => n.to_string(),
        };

        if let Some(res) = JS_GLUE_CODE_MAP.get(&key) {
            let v = res.value();
            String::from_utf8(v.to_vec()).ok()
        } else {
            None
        }
    }

    pub fn get_bytes_by_key(k: AnyGlueCodeKey) -> Option<Vec<u8>> {
        let key = match k {
            AnyGlueCodeKey::AutoInserted(n) => n.to_string(),
            AnyGlueCodeKey::Embeddable(n) => n.to_string(),
            AnyGlueCodeKey::General(n) => n.to_string(),
        };
        if let Some(b) = JS_GLUE_CODE_MAP.get(&key) {
            let x = b.value();
            Some(x.clone())
        } else {
            None
        }
    }
}

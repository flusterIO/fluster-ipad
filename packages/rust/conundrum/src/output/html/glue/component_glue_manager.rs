use crate::output::{
    general::component_constants::{
        auto_inserted_component_name::AutoInsertedComponentName, component_ids::EmbeddableComponentId,
        documentation_component_name::DocumentationComponentName,
    },
    html::glue::webglue_general_files::WebGlueCodeGeneralFiles,
};
use dashmap::DashMap;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

/// Just like the `AnyComponentName`, except this is unique where the
/// AnyComponentName allows for component aliases.
#[derive(Clone, Serialize, Deserialize, uniffi::Enum, Debug, PartialEq, Eq, Hash)]
#[serde(tag = "tag", content = "content")]
pub enum AnyComponentKey {
    AutoInserted(AutoInsertedComponentName),
    Embeddable(EmbeddableComponentId),
    General(WebGlueCodeGeneralFiles),
    Docs(DocumentationComponentName),
}

lazy_static! {
    pub static ref WEB_GLUE_CODE_MAP: DashMap<String, Vec<u8>> = {
        let map: DashMap<String, Vec<u8>> = DashMap::new();
        map.insert(format!("{}-css", WebGlueCodeGeneralFiles::Styles),
                   include_str!("../standalone/conundrum.css").to_string().as_bytes().to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::NerdFont),
                   include_bytes!("./component_glue_output/other/fonts/Fira_Code_Regular.ttf").to_vec());
        map.insert(format!("{}-css", WebGlueCodeGeneralFiles::KatexCss),
                   include_str!("../glue/component_glue/other/katex.min.css").to_string().as_bytes().to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_ams_regular),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_AMS-Regular.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_caligraphic_bold),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Caligraphic-Bold.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_caligraphic_regular),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Caligraphic-Regular.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_fraktur_bold),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Fraktur-Bold.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_fraktur_regular),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Fraktur-Regular.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_main_bold),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Main-Bold.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_main_bolditalic),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Main-BoldItalic.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_main_italic),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Main-Italic.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_main_regular),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Main-Regular.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_math_bolditalic),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Math-BoldItalic.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_math_italic),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Math-Italic.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_sansserif_bold),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_SansSerif-Bold.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_sansserif_italic),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_SansSerif-Italic.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_sansserif_regular),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_SansSerif-Regular.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_script_regular),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Script-Regular.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_size1_regular),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Size1-Regular.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_size2_regular),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Size2-Regular.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_size3_regular),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Size3-Regular.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_size4_regular),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Size4-Regular.woff2").to_vec());
        map.insert(format!("{}-font", WebGlueCodeGeneralFiles::Katex_typewriter_regular),
                   include_bytes!("./component_glue_output/other/fonts/KaTeX_Typewriter-Regular.woff2").to_vec());

        // --- Components ---
        map.insert(format!("{}-js", AutoInsertedComponentName::AutoInsertedCodeBlock),
                   include_str!("./component_glue_output/code_block.js").to_string().as_bytes().to_vec());
        map.insert(format!("{}-js", AutoInsertedComponentName::NoteLink),
                   include_str!("./component_glue_output/note_link.js").to_string().as_bytes().to_vec());
        map.insert(format!("{}-js", EmbeddableComponentId::Admonition), include_str!("./component_glue_output/admonition.js").to_string().as_bytes().to_vec());
        map.insert(format!("{}-css", EmbeddableComponentId::Admonition), include_str!("../../../lang/lib/ui/components/attention/admonition/admonition.css").to_string().as_bytes().to_vec());

        map.insert(format!("{}-js", EmbeddableComponentId::Tabs), include_str!("./component_glue_output/tabs.js").to_string().as_bytes().to_vec());

        map.insert(format!("{}-js", EmbeddableComponentId::Toc), include_str!("./component_glue_output/toc.js").to_string().as_bytes().to_vec());
        map
    };
}

impl WEB_GLUE_CODE_MAP {
    pub fn get_css_string_by_key(k: AnyComponentKey) -> Option<String> {
        let key = match k {
            AnyComponentKey::AutoInserted(n) => n.to_string(),
            AnyComponentKey::Embeddable(n) => n.to_string(),
            AnyComponentKey::General(n) => n.to_string(),
            AnyComponentKey::Docs(n) => n.to_string(),
        };

        if let Some(res) = WEB_GLUE_CODE_MAP.get(format!("{}-css", key).as_str()) {
            let v = res.value();
            String::from_utf8(v.to_vec()).ok()
        } else {
            eprintln!("Could not load resource for the {} key", key);
            None
        }
    }

    pub fn get_font_bytes_by_key(k: AnyComponentKey) -> Option<Vec<u8>> {
        let key = match k {
            AnyComponentKey::AutoInserted(n) => n.to_string(),
            AnyComponentKey::Embeddable(n) => n.to_string(),
            AnyComponentKey::General(n) => n.to_string(),
            AnyComponentKey::Docs(n) => n.to_string(),
        };
        if let Some(b) = WEB_GLUE_CODE_MAP.get(format!("{}-font", key).as_str()) {
            let x = b.value();
            Some(x.clone())
        } else {
            eprintln!("Could not load resource for the {} key", key);
            None
        }
    }

    pub fn get_js_string_by_key(k: AnyComponentKey) -> Option<String> {
        let key = match k {
            AnyComponentKey::AutoInserted(n) => n.to_string(),
            AnyComponentKey::Embeddable(n) => n.to_string(),
            AnyComponentKey::General(n) => n.to_string(),
            AnyComponentKey::Docs(n) => n.to_string(),
        };
        if let Some(res) = WEB_GLUE_CODE_MAP.get(format!("{}-js", key).as_str()) {
            let v = res.value();
            String::from_utf8(v.to_vec()).ok()
        } else {
            eprintln!("Could not load resource for the {} key", key);
            None
        }
    }
}

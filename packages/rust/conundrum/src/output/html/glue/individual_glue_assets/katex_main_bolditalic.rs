use crate::output::html::glue::{
    component_glue_manager::{AnyGlueCodeKey, JS_GLUE_CODE_MAP},
    glue_asset::GlueCssAsset,
    webglue_general_files::WebGlueCodeGeneralFiles,
};

pub struct KatexFontMainBoldItalic(WebGlueCodeGeneralFiles);

impl Default for KatexFontMainBoldItalic {
    fn default() -> Self {
        Self(WebGlueCodeGeneralFiles::Katex_main_bolditalic)
    }
}

impl GlueCssAsset for KatexFontMainBoldItalic {
    fn append_to_css(&self, content: &mut String, _: &bool) {
        if let Some(css_content) = JS_GLUE_CODE_MAP::get_string_by_key(AnyGlueCodeKey::General(self.0.clone())) {
            *content += css_content.as_str();
        } else {
            eprintln!("Could not find Katex css!!!");
        }
    }
}

use crate::output::html::glue::{
    component_glue_manager::{AnyGlueCodeKey, JS_GLUE_CODE_MAP, WebGlueCodeGeneralFiles},
    glue_asset::GlueCssAsset,
};

pub struct KatexCssAsset(WebGlueCodeGeneralFiles);

impl Default for KatexCssAsset {
    fn default() -> Self {
        Self(WebGlueCodeGeneralFiles::KatexCss)
    }
}

impl GlueCssAsset for KatexCssAsset {
    fn append_to_css(&self, content: &mut String, _: &bool) {
        if let Some(css_content) = JS_GLUE_CODE_MAP::get_by_key(AnyGlueCodeKey::General(self.0.clone())) {
            *content += css_content.as_str();
        } else {
            eprintln!("Could not find Katex css!!!");
        }
    }
}

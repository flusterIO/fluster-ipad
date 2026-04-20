use crate::output::html::glue::{
    component_glue_manager::{AnyGlueCodeKey, JS_GLUE_CODE_MAP},
    webglue_general_files::WebGlueCodeGeneralFiles,
};

pub fn font_to_css_embed(k: WebGlueCodeGeneralFiles) {
    if let Some(data) = JS_GLUE_CODE_MAP::get_bytes_by_key(AnyGlueCodeKey::General(k)) {
        let base64_str = base64::encode(&data);
        let css_str = format!("data:font/woff2;base64,{}", base64_str);
        // use css_str as needed
    }
}

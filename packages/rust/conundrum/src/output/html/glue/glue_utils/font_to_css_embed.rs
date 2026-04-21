use crate::output::html::glue::{
    component_glue_manager::{AnyComponentKey, WEB_GLUE_CODE_MAP},
    webglue_general_files::WebGlueCodeGeneralFiles,
};

pub fn font_to_css_embed(k: WebGlueCodeGeneralFiles) {
    if let Some(data) = WEB_GLUE_CODE_MAP::get_font_bytes_by_key(AnyComponentKey::General(k)) {
        let base64_str = base64::encode(&data);
        let css_str = format!("data:font/woff2;base64,{}", base64_str);
        // use css_str as needed
    }
}

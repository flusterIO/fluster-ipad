use askama::Template;
use base64::Engine;

use crate::output::html::glue::{
    component_glue_manager::{AnyComponentKey, WEB_GLUE_CODE_MAP},
    glue_asset::GlueCssAsset,
    webglue_general_files::WebGlueCodeGeneralFiles,
};

/// ## Template (HTML)
///
/// ```askama
/// @font-face {
/// font-family: 'FontNerd';
///  src: url('data:font/ttf;base64,{{get_font_data() | safe}}') format("truetype");
///    font-weight: normal;
///    font-style: normal;
///    font-display: block;
/// }
/// ```
#[derive(Template)]
#[template(ext = "html", escape = "none", in_doc = true)]
pub struct NerdFontAsset(WebGlueCodeGeneralFiles);

impl NerdFontAsset {
    fn get_font_data(&self) -> String {
        if let Some(data) = WEB_GLUE_CODE_MAP::get_font_bytes_by_key(AnyComponentKey::General(self.0.clone())) {
            // TODO: Lift this up the parent and pass it in so we don't have to recreate
            // this engine 20 times.
            let r = base64::engine::GeneralPurpose::new(&base64::alphabet::STANDARD,
                                                        base64::engine::GeneralPurposeConfig::new());
            r.encode(data)
        } else {
            eprintln!("The NerdFontAsset font could not be loaded");
            String::from("")
        }
    }
}

impl Default for NerdFontAsset {
    fn default() -> Self {
        Self(WebGlueCodeGeneralFiles::NerdFont)
    }
}

impl GlueCssAsset for NerdFontAsset {
    fn append_to_css(&self, content: &mut String, _: &bool) {
        if let Ok(res) = self.render() {
            *content += res.as_str();
        }
    }
}

use askama::Template;
use base64::Engine;

use crate::output::html::glue::{
    component_glue_manager::{AnyGlueCodeKey, JS_GLUE_CODE_MAP},
    glue_asset::GlueCssAsset,
    webglue_general_files::WebGlueCodeGeneralFiles,
};

/// ## Template (HTML)
/// ```askama
/// @font-face {
/// font-family: 'KaTeX_Main';
///  src: url('data:application/font-woff2;base64,{{get_font_data() | safe}}');
///    font-weight: bold;
///    font-style: normal;
///    font-display: block;
/// }
/// ```
#[derive(Template)]
#[template(ext = "html", escape = "none", in_doc = true)]
pub struct KatexFontAssetMainBold(WebGlueCodeGeneralFiles);

impl KatexFontAssetMainBold {
    fn get_font_data(&self) -> String {
        if let Some(data) = JS_GLUE_CODE_MAP::get_bytes_by_key(AnyGlueCodeKey::General(self.0.clone())) {
            // TODO: Lift this up the parent and pass it in so we don't have to recreate
            // this engine 20 times.
            let r = base64::engine::GeneralPurpose::new(&base64::alphabet::STANDARD,
                                                        base64::engine::GeneralPurposeConfig::new());
            r.encode(data)
        } else {
            eprintln!("The KatexFontAssetMainBold font could not be loaded");
            String::from("")
        }
    }
}

impl Default for KatexFontAssetMainBold {
    fn default() -> Self {
        Self(WebGlueCodeGeneralFiles::Katex_main_bold)
    }
}

impl GlueCssAsset for KatexFontAssetMainBold {
    fn append_to_css(&self, content: &mut String, _: &bool) {
        if let Ok(res) = self.render() {
            *content += res.as_str();
        }
    }
}


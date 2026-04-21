use askama::Template;
use base64::Engine;

use crate::output::html::glue::{
    component_glue_manager::{AnyComponentKey, WEB_GLUE_CODE_MAP},
    glue_asset::GlueCssAsset,
    webglue_general_files::WebGlueCodeGeneralFiles,
};

/// ## Template (HTML)
/// ```askama
/// @font-face {
/// font-family: 'KaTeX_AMS';
///  src: url('data:application/font-woff2;base64,{{get_font_data() | safe}}');
///  font-weight: normal;
///  font-style: normal;
///  font-display: block;
/// }
/// ```
#[derive(Template)]
#[template(ext = "html", escape = "none", in_doc = true)]
pub struct KatexFontAssetAmsRegular(WebGlueCodeGeneralFiles);

impl KatexFontAssetAmsRegular {
    fn get_font_data(&self) -> String {
        if let Some(data) = WEB_GLUE_CODE_MAP::get_font_bytes_by_key(AnyComponentKey::General(self.0.clone())) {
            // TODO: Lift this up the parent and pass it in so we don't have to recreate
            // this engine 20 times.
            let r = base64::engine::GeneralPurpose::new(&base64::alphabet::STANDARD,
                                                        base64::engine::GeneralPurposeConfig::new());
            r.encode(data)
        } else {
            eprintln!("The KatexFontAssetAmsRegular font could not be loaded");
            String::from("")
        }
    }
}

impl Default for KatexFontAssetAmsRegular {
    fn default() -> Self {
        Self(WebGlueCodeGeneralFiles::Katex_ams_regular)
    }
}

impl GlueCssAsset for KatexFontAssetAmsRegular {
    fn append_to_css(&self, content: &mut String, _: &bool) {
        if let Ok(res) = self.render() {
            *content += res.as_str();
        }
    }
}

use askama::Template;
use base64::Engine;

use crate::output::html::glue::glue_asset::GlueCssAsset;

/// ## Template (HTML)
/// ```askama
/// @font-face {
/// font-family: 'FontLucide';
///  src: url('data:font/ttf;base64,{{get_font_data() | safe}}') format("truetype");
///    font-weight: normal;
///    font-style: normal;
///    font-display: block;
/// }
/// ```
#[derive(Template)]
#[template(ext = "html", escape = "none", in_doc = true)]
pub struct LucideFontAsset {}

impl LucideFontAsset {
    fn get_font_data(&self) -> String {
        // TODO: Lift this up the parent and pass it in so we don't have to recreate
        // this engine 20 times.
        let r = base64::engine::GeneralPurpose::new(&base64::alphabet::STANDARD,
                                                    base64::engine::GeneralPurposeConfig::new());
        r.encode(lucide_icons::LUCIDE_FONT_BYTES)
    }
}

impl GlueCssAsset for LucideFontAsset {
    fn append_to_css(&self, content: &mut String, _: &bool) {
        if let Ok(res) = self.render() {
            *content += res.as_str();
        }
    }
}

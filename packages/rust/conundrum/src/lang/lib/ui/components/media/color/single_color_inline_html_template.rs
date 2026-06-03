use askama::Template;

use crate::{
    output::html::web_specific_traits::css_value_representable::CSSInlineHtmlValuePairRepresentable,
    parsers::conundrum::color::conundrum_color::ConundrumColor,
};

/// ## Template (HTML)
///
/// ```askama
/// <span class="w-4 h-4 rounded border" style="background-color: {{self.0.as_inline_style_value_group().background | safe}}">
/// </span>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct SingleColorInlineHtmlTemplate(ConundrumColor);

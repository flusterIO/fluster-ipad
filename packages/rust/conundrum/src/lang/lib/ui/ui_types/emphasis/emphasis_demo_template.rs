use askama::Template;

use crate::{
    lang::lib::ui::ui_types::emphasis::emphasis_model::Emphasis,
    output::html::web_specific_traits::css_value_representable::CSSInlineHtmlValuePairRepresentable,
};

/// ## Template (HTML)
///
/// ```askama
/// <div class="w-fit flex flex-col justify-center items-center bg-fd-card text-fd-card-foreground rounded border">
/// {% let styles_group = self.0.as_inline_style_value_group() %}
/// <div class="w-full h-full min-w-32 min-h-32 rounded border" style="background-color: {{styles_group.background}};text-color: {{styles_group.foreground}};">
/// </div>
/// <div class="w-fit text-center overflow-y-hidden overflow-x-auto no-scrollbar">
/// {{self.0.to_string()}}
/// </div>
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct EmphasisDemoTemplate(pub Emphasis);

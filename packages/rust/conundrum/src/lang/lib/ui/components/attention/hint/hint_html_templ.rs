use crate::lang::lib::ui::{shared_props::sizable::SizablePropsGroup, ui_types::emphasis::Emphasis};
use askama::Template;
use tw_merge::*;

/// Template (HTML)
///
/// ```askama
/// <div class="{{tw_merge!("text-sm mt-2 mb-6", self.size.as_ref().cloned().map(|s| s.as_class()).unwrap_or_default())}}">
///  <span class="{{tw_merge!("font-semibold pr-2", self.emphasis.to_foreground_css_classes())}}">
///  {{label | safe}}
///  </span>
///  <span class="[&>*]:inline! text-foreground/80 [&>*]:text-foreground/80 [&>*]:text-sm [&>p]:leading-tight text-sm leading-tight">
///  {{body | safe}}
///  </span>
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct HintHtmlTemplate {
    pub emphasis: Emphasis,
    pub size: Option<SizablePropsGroup>,
    pub label: String,
    pub body: String,
}

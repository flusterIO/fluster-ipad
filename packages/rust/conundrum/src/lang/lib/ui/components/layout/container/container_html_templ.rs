use crate::lang::lib::ui::{shared_props::sizable::SizablePropsGroup, ui_types::emphasis::Emphasis};
use askama::Template;
use tw_merge::*;

/// ## Template (HTML)
///
/// ```askama
/// <div class="{{tw_merge!(self.emphasis.as_ref().cloned().map(|emph| emph.to_background_color_classes()).unwrap_or_default(), self.sizable.as_class())}}">
/// {{children | safe}}
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct UtilityContainerHtmlTemplate {
    pub children: String,
    pub emphasis: Option<Emphasis>,
    pub sizable: SizablePropsGroup,
}

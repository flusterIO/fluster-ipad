use askama::Template;

use crate::{
    lang::lib::ui::shared_props::{sizable::SizablePropsGroup, sizable_option::SizableOption},
    output::html::css::css_classlist::CSSClassList,
};

/// ## Template (HTML)
///
/// ```askama
/// <div class="{{container_classes}}">
///    My content
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct CodeBlockHTMLTemplate {
    pub container_classes: SizablePropsGroup,
}

impl Default for CodeBlockHTMLTemplate {
    fn default() -> Self {
        CodeBlockHTMLTemplate { container_classes: SizablePropsGroup { width: Some(SizableOption::Full),
                                                                       ..Default::default() } }
    }
}

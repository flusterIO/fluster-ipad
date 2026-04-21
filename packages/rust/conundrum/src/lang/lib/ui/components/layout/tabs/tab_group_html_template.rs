use askama::Template;

use crate::lang::lib::ui::{
    components::layout::tabs::tab_html_template::TabButtonHtmlTemplate, shared_props::sizable::SizablePropsGroup,
    ui_types::emphasis::Emphasis,
};

#[derive(Template)]
#[template(ext = "html", path = "tab_group.html")]
pub struct TabGroupHtmlTemplate {
    /// The _rendered_ children.
    pub children: String,
    pub tab_group_id: String,
    pub subtle: bool,
    pub sizable: Option<SizablePropsGroup>,
    pub emphasis: Emphasis,
    pub tabs: Vec<TabButtonHtmlTemplate>,
}

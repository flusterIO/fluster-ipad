use askama::Template;

use crate::{
    lang::runtime::state::ui_params::UIParams,
    parsers::markdown::markdown_extensions::footnote::footnote_footer_html_templ::FootnoteSectionTemplate,
};

#[derive(Template)]
#[template(ext = "jinja", path = "standalone.jinja")]
pub struct StandaloneTemplate {
    pub label: Option<String>,
    pub css: String,
    pub js: String,
    pub body: String,
    pub ui: UIParams,
    pub footnotes: FootnoteSectionTemplate,
}

impl StandaloneTemplate {
    pub fn new(label: Option<String>,
               body: String,
               footnotes: FootnoteSectionTemplate,
               js: String,
               css: String,
               ui: UIParams)
               -> Self {
        // let font_s = include_str!("../../../../assets/fonts/Fira Code Regular Nerd
        // Font Complete.ttf");
        StandaloneTemplate { label,
                             css,
                             footnotes,
                             js,
                             body,
                             ui }
    }
}

use askama::Template;

use crate::{
    lang::runtime::state::ui_params::UIParams,
    output::html::glue::component_glue_manager::{JS_GLUE_CODE_MAP, WebGlueCodeGeneralFiles},
};

#[derive(Template)]
#[template(ext = "jinja", path = "standalone.jinja")]
pub struct StandaloneTemplate {
    pub label: Option<String>,
    pub css: String,
    pub js: String,
    pub body: String,
    pub ui: UIParams,
}

impl StandaloneTemplate {
    pub fn new(label: Option<String>, body: String, js: String, ui: UIParams) -> Self {
        let css =
            JS_GLUE_CODE_MAP.get(WebGlueCodeGeneralFiles::Styles.to_string().as_str()).expect("Css better be there...");
        StandaloneTemplate { label,
                             css: css.to_string(),
                             js,
                             body,
                             ui }
    }
}

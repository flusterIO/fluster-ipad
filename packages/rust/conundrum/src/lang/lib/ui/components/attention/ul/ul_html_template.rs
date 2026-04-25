use askama::Template;

use crate::{lang::lib::ui::ui_types::emphasis::Emphasis, parsers::conundrum::logic::bool::boolean::ConundrumBoolean};

/// Template (HTML)
///
/// ```askama
/// <span class="underline [&_*]:text-inerhit {{emphasis.to_text_decodartion_classes()}}
/// {{self.get_thickness_classes()}}">{{content}}</span>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct UlHtmlTemplate {
    pub content: String,
    pub emphasis: Emphasis,
    pub thick: Option<ConundrumBoolean>,
    pub thin: Option<ConundrumBoolean>,
}

impl UlHtmlTemplate {
    pub fn get_thickness_classes(&self) -> String {
        if self.thick.is_some_and(|x| x.0) {
            String::from("decoration-4")
        } else if self.thin.is_some_and(|x| x.0) {
            String::from("decordation-1")
        } else {
            String::from("decoration-2")
        }
    }
}

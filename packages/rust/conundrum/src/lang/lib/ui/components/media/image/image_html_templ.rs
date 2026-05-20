use askama::Template;
use tw_merge::*;

use crate::{
    lang::lib::ui::shared_props::sizable::SizablePropsGroup, output::html::dom::dom_id::DOMId,
    parsers::conundrum::logic::string::conundrum_string::ConundrumString,
};

/// ## Template (HTML)
/// ```askama
/// <img
/// src="{{src | safe}}"
/// class="{{tw_merge!("max-h-full max-w-full", self.sizable.as_class(crate::lang::lib::ui::shared_props::sizable::SizablePropsOutputTarget::Image), self.get_object_fit_class()) | safe}}"
/// id="{{id | safe}}"
/// ></img>
/// ```
#[derive(Template)]
#[template(in_doc = true, ext = "html")]
pub struct ImageHtmlTemplate {
    pub id: DOMId,
    pub sizable: SizablePropsGroup,
    pub src: ConundrumString,
    pub alt: Option<ConundrumString>,
    pub contain: bool,
    pub cover: bool,
}

impl ImageHtmlTemplate {
    pub fn get_object_fit_class(&self) -> String {
        if self.contain {
            String::from("object-contain")
        } else if self.cover {
            String::from("object-cover")
        } else {
            String::from("")
        }
    }
}

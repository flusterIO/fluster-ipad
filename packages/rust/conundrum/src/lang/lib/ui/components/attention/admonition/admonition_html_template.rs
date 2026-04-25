use crate::{
    lang::lib::ui::{shared_props::sizable::SizablePropsGroup, ui_types::emphasis::Emphasis},
    output::html::dom::dom_id::DOMId,
    parsers::conundrum::logic::bool::boolean::ConundrumBoolean,
};
use askama::Template;
use tw_merge::*;

#[derive(Template)]
#[template(ext = "html", path = "admonition.html")]
pub struct AdmonitionHtmlTemplate {
    pub sizable: SizablePropsGroup,
    pub foldable: ConundrumBoolean,
    pub folded: ConundrumBoolean,
    pub title_children: String,
    pub body_children: String,
    pub emphasis: Emphasis,
    pub id: DOMId,
}

impl AdmonitionHtmlTemplate {
    pub fn get_icon(&self) -> String {
        let icon = self.emphasis.to_icon();
        icon.unicode().to_string()
    }

    pub fn chevron(&self) -> String {
        let s = lucide_icons::Icon::ChevronUp.unicode().to_string();

        s
    }

    pub fn is_center_self(&self) -> bool {
        self.sizable.center_self.is_some_and(|a| a.0)
    }
}

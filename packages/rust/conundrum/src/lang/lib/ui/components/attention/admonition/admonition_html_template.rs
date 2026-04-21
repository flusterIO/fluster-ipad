use crate::{
    lang::lib::ui::{shared_props::sizable::SizablePropsGroup, ui_types::emphasis::Emphasis},
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
}

impl AdmonitionHtmlTemplate {
    pub fn get_icon(&self) -> String {
        let icon = self.emphasis.to_icon();
        icon.unicode().to_string()
    }

    pub fn chevron(&self) -> String {
        let s = lucide_icons::Icon::ChevronUp.unicode().to_string();

        println!("S: {:#?}", s);
        s
    }

    pub fn is_center_self(&self) -> bool {
        self.sizable.center_self.is_some_and(|a| a.0)
    }
}

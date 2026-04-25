use askama::Template;

use crate::{
    lang::lib::ui::components::layout::toc::create_nested_toc::{TocItemWithNested, get_toc_nested_items},
    output::html::dom::dom_id::DOMId,
    parsers::markdown::heading::heading_model::MarkdownHeadingStringifiedResult,
};

#[derive(Template)]
#[template(ext = "html", path = "toc/table-of-contents-item.html")]
pub struct TocItemTemplate {
    pub item: MarkdownHeadingStringifiedResult,
    pub expanded: bool,
    pub group_id: DOMId,
}

impl TocItemTemplate {
    pub fn padding_left(&self) -> f32 {
        (self.item.tab_depth as f32) * 0.5
    }
}

pub enum TocItemVariant {
    WithNested(TocItemWithNested),
    NoNested(TocItemTemplate),
}

#[derive(Template)]
#[template(ext = "html", path = "toc/table-of-contents.html")]
pub struct TocHtmlTemplate {
    pub toc: Vec<MarkdownHeadingStringifiedResult>,
    pub expanded: bool,
    pub id: DOMId,
}

impl TocHtmlTemplate {
    pub fn get_items(&self) -> Vec<TocItemVariant> {
        let nested_items = get_toc_nested_items(self.toc.clone(), self.expanded, self.id.clone());
        let mut items: Vec<TocItemVariant> = Vec::new();
        for item in nested_items.iter() {
            if item.nested.is_empty() {
                items.push(TocItemVariant::NoNested(TocItemTemplate { item: item.item.clone(),
                                                                      expanded: self.expanded,
                                                                      group_id: self.id.clone() }));
            } else {
                items.push(TocItemVariant::WithNested(item.clone()));
            }
        }
        items
    }
}

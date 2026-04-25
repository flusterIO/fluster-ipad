use askama::Template;

use crate::{
    output::html::dom::dom_id::DOMId, parsers::markdown::heading::heading_model::MarkdownHeadingStringifiedResult,
};

#[derive(Template, Clone)]
#[template(ext = "html", path = "toc/table-of-contents-item-with-nested.html")]
pub struct TocItemWithNested {
    pub item: MarkdownHeadingStringifiedResult,
    pub expanded: bool,
    pub group_id: DOMId,
    pub nested: Vec<TocItemWithNested>,
}

impl TocItemWithNested {
    pub fn padding_left(&self) -> f32 {
        (self.item.tab_depth as f32) * 0.5
    }
}

fn consume_children(items: Vec<MarkdownHeadingStringifiedResult>,
                    expanded: bool,
                    group_id: DOMId)
                    -> Option<(TocItemWithNested, Vec<MarkdownHeadingStringifiedResult>)> {
    if items.is_empty() {
        return None;
    }
    let mut first_item = TocItemWithNested { item: items.first().unwrap().clone(),
                                             expanded,
                                             group_id: group_id.clone(),
                                             nested: Vec::new() };
    let mut remaining = items.iter().skip(1).cloned().collect::<Vec<MarkdownHeadingStringifiedResult>>();
    for k in remaining.clone() {
        if k.depth > first_item.item.depth {
            if let Some((new_k, new_remaining)) = consume_children(remaining.clone(), expanded, group_id.clone()) {
                first_item.nested.push(new_k);
                remaining = new_remaining;
            }
        } else {
            return Some((first_item, remaining));
        }
    }
    Some((first_item, remaining))
}

pub fn get_toc_nested_items(_toc: Vec<MarkdownHeadingStringifiedResult>,
                            expanded: bool,
                            group_id: DOMId)
                            -> Vec<TocItemWithNested> {
    println!("Toc: {:#?}", _toc);
    if _toc.is_empty() {
        Vec::new()
    } else {
        let mut items = Vec::new();
        let mut toc = _toc.clone();
        loop {
            if let Some((new_item, remaining)) = consume_children(toc.clone(), expanded, group_id.clone()) {
                items.push(new_item);
                toc = remaining;
            } else if toc.is_empty() {
                break;
            }
        }
        items
    }
}

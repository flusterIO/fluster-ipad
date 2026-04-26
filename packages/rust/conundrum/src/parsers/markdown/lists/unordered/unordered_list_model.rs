use std::sync::Arc;

use askama::Template;
use serde::Serialize;
use winnow::{
    Parser,
    combinator::{alt, repeat},
    error::ErrMode,
};

use crate::{
    lang::runtime::{
        state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        },
        traits::{
            as_template::AsTemplate,
            conundrum_input::{ArcState, ConundrumInput},
            fluster_component_result::ConundrumComponentResult,
            html_js_component_result::HtmlJsComponentResult,
        },
    },
    parsers::{
        markdown::lists::unordered::unordered_line_item::{
            unorder_list_html_templ::{UnorderedListHtmlTemplate, UnorderedListItemHtmlTemplate},
            unordered_line_item_model::UnorderedListItem,
        },
        parser_trait::ConundrumParser,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct UnorderedListModel {
    pub items: Vec<UnorderedListItem>,
}

impl ConundrumComponentResult for UnorderedListModel {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        // TODO: This is obviously completely incomplete and needs to be handled for all
        // other targets. I'm homeless, and I need a paycheck before this
        // macbook takes a s--t on me and I'm really s-o-l, so I'm moving on.
        self.to_html_js_component(res)
    }
}

impl HtmlJsComponentResult for UnorderedListModel {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let mut new_items: Vec<UnorderedListItemHtmlTemplate> = Vec::new();

        for item in &self.items {
            let x = item.as_template(Arc::clone(&res))?;
            new_items.push(x);
        }

        let templ = UnorderedListHtmlTemplate { items: new_items };

        templ.render().map_err(|e| {
                eprintln!("Error: {:#?}", e);
                ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
            })
    }
}

impl ConundrumParser<UnorderedListModel> for UnorderedListModel {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<UnorderedListModel> {
        let res = repeat(1.., UnorderedListItem::parse_input_string).parse_next(input)?;

        Ok(UnorderedListModel { items: res })
    }

    fn matches_first_char(char: char) -> bool {
        char == '+' || char == '*' || char == '-'
    }
}

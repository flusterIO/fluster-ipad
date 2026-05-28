use std::sync::Arc;

use askama::Template;
use serde::Serialize;
use winnow::{Parser, combinator::repeat, error::ErrMode};

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
        markdown::lists::task_list::{
            task_list_html_templ::UnorderedTaskListHtmlTemplate,
            task_list_item::{
                task_list_item_html_templ::UnorderedTaskListItemHtmlTemplate,
                task_list_item_model::UnorderedTaskListItem,
            },
        },
        parser_trait::ConundrumParser,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, serde::Deserialize, Clone)]
pub struct UnorderedTaskListModel {
    pub items: Vec<UnorderedTaskListItem>,
}

impl ConundrumComponentResult for UnorderedTaskListModel {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.to_html_js_component(res)
    }
}

impl HtmlJsComponentResult for UnorderedTaskListModel {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let mut new_items: Vec<UnorderedTaskListItemHtmlTemplate> = Vec::new();

        for item in &self.items {
            let x = item.as_template(Arc::clone(&res))?;
            new_items.push(x);
        }

        let templ = UnorderedTaskListHtmlTemplate { items: new_items };

        templ.render().map_err(|e| {
                eprintln!("Error: {:#?}", e);
                ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
            })
    }
}

impl ConundrumParser<UnorderedTaskListModel> for UnorderedTaskListModel {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<UnorderedTaskListModel> {
        let res = repeat(1.., UnorderedTaskListItem::parse_input_string).parse_next(input)?;

        Ok(UnorderedTaskListModel { items: res })
    }

    fn matches_first_char(char: char) -> bool {
        char == '+' || char == '*' || char == '-'
    }
}

use std::sync::Arc;

use askama::Template;
use serde::Serialize;
use winnow::combinator::repeat;
use winnow::error::ErrMode;
use winnow::{Parser, stream::AsChar};

use crate::lang::runtime::state::conundrum_error::ConundrumError;
use crate::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;
use crate::lang::runtime::traits::conundrum_template::{
    ConundrumTemplateRepresentable, ConundrumTemplateRepresentableWithParam,
};
use crate::parsers::markdown::lists::ordered::ordered_list_html_templ::OrderedListHtmlTemplate;
use crate::{
    lang::runtime::{
        state::conundrum_error_variant::ConundrumModalResult,
        traits::{
            conundrum_input::{ArcState, ConundrumInput},
            fluster_component_result::ConundrumComponentResult,
            html_js_component_result::HtmlJsComponentResult,
            plain_text_component_result::PlainTextComponentResult,
        },
    },
    parsers::{
        markdown::lists::ordered::ordered_list_item::ordered_list_item_model::OrderedListItem,
        parser_trait::ConundrumParser,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, serde::Deserialize, Clone)]
pub struct OrderedListModel {
    pub items: Vec<OrderedListItem>,
}

impl PlainTextComponentResult for OrderedListModel {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        todo!()
    }
}

impl ConundrumComponentResult for OrderedListModel {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        todo!()
    }
}

impl ConundrumTemplateRepresentable<OrderedListHtmlTemplate> for OrderedListModel {
    fn to_template(&self, state: ArcState) -> ConundrumModalResult<OrderedListHtmlTemplate> {
        let mut items = Vec::new();
        for (idx, item) in self.items.iter().enumerate() {
            let r = item.to_template(Arc::clone(&state), idx as i32)?;
            items.push(r);
        }
        Ok(OrderedListHtmlTemplate { items })
    }
}

impl HtmlJsComponentResult for OrderedListModel {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.to_template(Arc::clone(&res))?.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl ConundrumParser<OrderedListModel> for OrderedListModel {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<OrderedListModel> {
        let items = repeat(1.., OrderedListItem::parse_input_string).parse_next(input)?;
        Ok(OrderedListModel { items })
    }

    fn matches_first_char(char: char) -> bool {
        char.is_dec_digit()
    }
}

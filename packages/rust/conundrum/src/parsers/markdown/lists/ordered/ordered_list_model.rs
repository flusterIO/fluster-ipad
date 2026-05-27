use serde::Serialize;
use winnow::combinator::repeat;
use winnow::{Parser, stream::AsChar};

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

impl HtmlJsComponentResult for OrderedListModel {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        todo!()
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

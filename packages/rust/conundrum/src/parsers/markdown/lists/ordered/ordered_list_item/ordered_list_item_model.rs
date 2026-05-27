use serde::{Deserialize, Serialize};
use winnow::{
    Parser,
    ascii::dec_int,
    combinator::alt,
    stream::{AsChar, Stream},
};

use crate::{
    lang::{
        lib::ui::ui_types::children::Children,
        runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    },
    parsers::{
        markdown::lists::unordered::unordered_line_item::unordered_line_item_model::parse_list_item_children,
        parser_trait::ConundrumParser, parsers_shared::space_or_tab::spaces_only,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderedListItem {
    pub heading: Children,
    pub body: Option<Children>,
    /// n will be none if the `n.` syntax was used instead of `1.`.
    pub n: Option<i32>,
}

pub fn markdown_ordered_list_terminator(input: &mut ConundrumInput) -> ConundrumModalResult<char> {
    alt(('.', ')')).parse_next(input)
}

impl ConundrumParser<OrderedListItem> for OrderedListItem {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<OrderedListItem> {
        let start = input.input.checkpoint();
        let n = alt((dec_int.map(Some), 'n'.value(None))).parse_next(input).inspect_err(|_| {
                                                                                input.input.reset(&start);
                                                                            })?;
        markdown_ordered_list_terminator.void().parse_next(input).inspect_err(|_| {
                                                                      input.input.reset(&start);
                                                                  })?;
        spaces_only(1).parse_next(input).inspect_err(|_| {
                                             input.input.reset(&start);
                                         })?;

        let (heading, body) = parse_list_item_children.parse_next(input).inspect_err(|_| {
                                                                             input.input.reset(&start);
                                                                         })?;
        Ok(OrderedListItem { heading,
                             body,
                             n })
    }

    fn matches_first_char(char: char) -> bool {
        char.is_dec_digit()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_basic_ordered_list_item() {
        let mut input = wrap_test_conundrum_content("1. My test item goes here.");
        let res = OrderedListItem::parse_input_string.parse_next(&mut input)
                                                     .expect("Parses the list item without throwing an error.");

        assert!(res.n.is_some_and(|n| n == 1), "Finds the proper index");
        assert!(res.heading.render(Arc::clone(&input.state)).expect("Renders ordered list item children as expected.")
                == "My test item goes here.",
                "Finds the proper heading.");
        assert!(res.body.is_none(), "Finds no body when none exists");
    }

    #[test]
    fn parses_multi_line_ordered_list_item() {
        let mut input = wrap_test_conundrum_content(
                                                    r#"1. My test item goes here.
  My list item's body continues here."#,
        );
        let res = OrderedListItem::parse_input_string.parse_next(&mut input)
                                                     .expect("Parses the list item without throwing an error.");

        assert!(res.n.is_some_and(|n| n == 1), "Finds the proper index");
        assert!(res.heading.render(Arc::clone(&input.state)).expect("Renders ordered list item children as expected.")
                == "My test item goes here.",
                "Finds the proper heading.");
        assert!(res.body.is_some(), "Finds a body when one exists");
    }
}

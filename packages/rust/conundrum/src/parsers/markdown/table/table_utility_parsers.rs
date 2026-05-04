use std::fmt::Debug;

use crate::{
    lang::runtime::{
        state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        },
        traits::conundrum_input::ConundrumInput,
    },
    parsers::{
        parser_components::white_space_delimited::space_or_tab_delimited,
        parsers_shared::{escape_handling::ESCAPE_CHAR, line_breaks::any_line_break},
    },
};

use winnow::{
    Parser,
    combinator::{alt, delimited, not, separated},
    error::{ContextError, ErrMode},
};

pub fn table_separator_not_escaped(input: &mut ConundrumInput) -> ConundrumModalResult<()> {
    (not(ESCAPE_CHAR), '|').void()
        .map_err(|_: ContextError| {
            ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Not a table separator.")))
        })
        .parse_next(input)
}

pub fn table_cell_breaking_element(input: &mut ConundrumInput) -> ConundrumModalResult<()> {
    alt(('|'.void(), any_line_break)).parse_next(input)
}

fn table_row_parser_wrapper_inner<'a, T>(parser: impl FnMut(&mut ConundrumInput<'a>) -> ConundrumModalResult<T> + Clone)
                                         -> impl FnMut(&mut ConundrumInput<'a>) -> ConundrumModalResult<Vec<T>> {
    move |input| {
        let r: Vec<T> = separated(1.., space_or_tab_delimited(parser.clone()), '|').parse_next(input)?;
        Ok(r)
    }
}

pub fn table_row_parser_wrapper<'a, T: Debug>(
    parser: impl Fn(&mut ConundrumInput<'a>) -> ConundrumModalResult<T> + Clone)
    -> impl FnMut(&mut ConundrumInput<'a>) -> ConundrumModalResult<Vec<T>> {
    move |input| {
        let res: Vec<T> =
            delimited('|',
                      space_or_tab_delimited(table_row_parser_wrapper_inner(parser.clone())),
                    '|').parse_next(input)?;
        // (consume_linear_space(0..), alt((take_while(1.., AsChar::is_newline), eof)))
        println!("Input: {:#?}", res);
        Ok(res)
    }
}

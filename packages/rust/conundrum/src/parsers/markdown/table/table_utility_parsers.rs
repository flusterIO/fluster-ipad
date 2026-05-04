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
        parser_components::consume_white_space::consume_linear_space,
        parsers_shared::{
            escape_handling::ESCAPE_CHAR,
            line_breaks::{any_line_break, white_space_to_newline},
            space_or_tab::space_or_tab,
        },
    },
};

use winnow::{
    Parser,
    combinator::{alt, delimited, not, opt, repeat, repeat_till},
    error::{ContextError, ErrMode},
    stream::Stream,
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

pub fn terminating_whitespace_and_table_separator(input: &mut ConundrumInput) -> ConundrumModalResult<()> {
    (space_or_tab(0..), '|').void().parse_next(input)
}

fn table_row_parser_wrapper_inner<'a, T>(parser: impl FnMut(&mut ConundrumInput<'a>) -> ConundrumModalResult<T> + Clone)
                                         -> impl FnMut(&mut ConundrumInput<'a>) -> ConundrumModalResult<Vec<T>> {
    move |input| {
        let start = input.input.checkpoint();
        consume_linear_space(0..).parse_next(input).inspect_err(|_| {
                                                        input.input.reset(&start);
                                                    })?;
        let (ts, _): (Vec<T>, ()) =
            repeat_till(1.., parser.clone(), terminating_whitespace_and_table_separator.void()).parse_next(input)
                                                                                               .inspect_err(|_| {
                                                                                                   input.input
                                                                                                        .reset(&start);
                                                                                               })?;

        // let
        Ok(ts)
    }
}

pub fn table_row_parser_wrapper<'a, T: Debug>(
    parser: impl Fn(&mut ConundrumInput<'a>) -> ConundrumModalResult<T> + Clone)
    -> impl FnMut(&mut ConundrumInput<'a>) -> ConundrumModalResult<Vec<T>> {
    move |input| {
        let res: Vec<T> = delimited('|', repeat(1.., parser.clone()), white_space_to_newline).parse_next(input)?;
        // (consume_linear_space(0..), alt((take_while(1.., AsChar::is_newline), eof)))
        println!("Input: {:#?}", res);
        Ok(res)
    }
}

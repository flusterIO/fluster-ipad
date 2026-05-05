use std::fmt::Debug;

use crate::{
    lang::runtime::{
        state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        },
        traits::conundrum_input::ConundrumInput,
    },
    parsers::parsers_shared::{
        escape_handling::ESCAPE_CHAR,
        line_breaks::{any_line_break, white_space_to_newline},
        space_or_tab::space_or_tab,
    },
};

use winnow::{
    Parser,
    combinator::{alt, delimited, not, repeat},
    error::{ContextError, ErrMode},
    stream::Range,
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

pub fn table_row_parser_wrapper<'a, T: Debug>(
    parser: impl Fn(&mut ConundrumInput<'a>) -> ConundrumModalResult<T> + Clone,
    col_count: Option<usize>)
    -> impl FnMut(&mut ConundrumInput<'a>) -> ConundrumModalResult<Vec<T>> {
    let r: Range = match col_count {
        Some(s) => s.into(),
        None => (0..).into(),
    };
    move |input| {
        let res: Vec<T> = delimited('|', repeat(r, parser.clone()), white_space_to_newline).parse_next(input)?;
        Ok(res)
    }
}

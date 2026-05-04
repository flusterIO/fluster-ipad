use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    parsers::{
        parser_components::consume_white_space::consume_linear_space, parsers_shared::escape_handling::ESCAPE_CHAR,
    },
};
use winnow::{
    Parser,
    combinator::{alt, eof, not},
};
use winnow::{ascii::line_ending, stream::Stream, token::literal};

pub fn any_line_break(input: &mut ConundrumInput) -> ConundrumModalResult<()> {
    alt(('\n'.void(), literal("\r\n").void())).parse_next(input)
}

pub fn line_break_not_escaped(input: &mut ConundrumInput) -> ConundrumModalResult<()> {
    (not(ESCAPE_CHAR), any_line_break).void().parse_next(input)
}

pub fn white_space_to_newline(input: &mut ConundrumInput) -> ConundrumModalResult<()> {
    let start = input.input.checkpoint();
    consume_linear_space(0..).parse_next(input).inspect_err(|_| {
                                                    input.input.reset(&start);
                                                })?;
    alt((line_ending, eof)).parse_next(input).inspect_err(|_| {
                                                  input.input.reset(&start);
                                              })?;
    Ok(())
}

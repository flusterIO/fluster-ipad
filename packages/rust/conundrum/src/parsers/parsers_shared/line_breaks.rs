use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    parsers::parsers_shared::escape_handling::ESCAPE_CHAR,
};
use winnow::token::literal;
use winnow::{
    Parser,
    combinator::{alt, not},
};

pub fn any_line_break(input: &mut ConundrumInput) -> ConundrumModalResult<()> {
    alt(('\n'.void(), literal("\r\n").void())).parse_next(input)
}

pub fn line_break_not_escaped(input: &mut ConundrumInput) -> ConundrumModalResult<()> {
    (not(ESCAPE_CHAR), any_line_break).void().parse_next(input)
}

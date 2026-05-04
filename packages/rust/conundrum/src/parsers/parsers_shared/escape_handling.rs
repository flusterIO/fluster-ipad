pub static ESCAPE_CHAR: char = '\\';
use winnow::{Parser, token::take};

use crate::lang::runtime::{
    state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput,
};

pub fn escaped_char<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<&'a str> {
    (take(1usize).void(), take(1usize)).map(|(_, c): (_, &'a str)| c).parse_next(input)
}

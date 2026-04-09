use crate::lang::runtime::{
    state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput,
};
use winnow::{Parser, combinator::alt};

pub fn markdown_bold_italic_anchor<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<String> {
    alt(('*', '_')).map(|c| c.to_string()).parse_next(input)
}

use winnow::token::literal;
use winnow::{
    Parser,
    combinator::{alt, repeat},
    stream::Range,
};

use crate::lang::runtime::{
    state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput,
};

pub fn indentation(input: &mut ConundrumInput) -> ConundrumModalResult<String> {
    alt((literal("  "), literal("\t"))).map(String::from).parse_next(input)
}

/// This **consumes** the indented space.
pub fn indentation_count(occurences: impl Into<Range> + Clone)
                         -> impl Fn(&mut ConundrumInput) -> ConundrumModalResult<(String, usize)> {
    move |input: &mut ConundrumInput| {
        let res: Vec<String> = repeat(occurences.clone(), indentation).parse_next(input)?;

        Ok((res.join(""), res.len()))
    }
}

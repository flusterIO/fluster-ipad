use crate::lang::runtime::{
    state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput,
};
use winnow::{
    Parser,
    combinator::delimited,
    token::{literal, take_while},
};

pub fn inline_id_syntax(input: &mut ConundrumInput) -> ConundrumModalResult<String> {
    delimited(literal("{#"), take_while(1.., |c: char| c.is_alphanumeric() || c == '-' || c == '_'), '}').map(String::from).parse_next(input)
}

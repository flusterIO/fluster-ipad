use crate::lang::runtime::{
    state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput,
};
use winnow::{Parser, combinator::alt};

pub fn line_item_marker(input: &mut ConundrumInput) -> ConundrumModalResult<()> {
    alt(('-', '+', '*')).void().parse_next(input)
}

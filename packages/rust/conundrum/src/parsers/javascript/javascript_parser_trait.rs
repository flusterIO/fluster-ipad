use crate::lang::runtime::{
    state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput,
};

pub trait JavascriptParser<T> {
    fn parse_javascript(input: &mut ConundrumInput) -> ConundrumModalResult<T>;
}

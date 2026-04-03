use winnow::ModalResult;

use crate::lang::runtime::{state::conundrum_error_variant::ConundrumResult, traits::conundrum_input::ConundrumInput};

pub trait JavascriptParser<T> {
    fn parse_javascript(input: &mut ConundrumInput) -> ConundrumResult<T>;
}

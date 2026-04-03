use winnow::ModalResult;

use crate::lang::runtime::{state::conundrum_error_variant::ConundrumResult, traits::conundrum_input::ConundrumInput};

pub trait ConundrumParser<T> {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumResult<T>;
    fn matches_first_char(char: char) -> bool;
}

use winnow::ModalResult;

use crate::lang::runtime::traits::conundrum_input::ConundrumInput;

pub trait ConundrumParser<T> {
    fn parse_input_string(input: &mut ConundrumInput) -> ModalResult<T>;
    fn matches_first_char(char: char) -> bool;
}

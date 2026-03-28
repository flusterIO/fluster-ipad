use winnow::ModalResult;

use crate::lang::runtime::traits::conundrum_input::ConundrumInput;

pub trait JavascriptParser<T> {
    fn parse_javascript(input: &mut ConundrumInput) -> ModalResult<T>;
}

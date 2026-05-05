use crate::lang::runtime::{
    state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput,
};

pub trait ConundrumParser<T> {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<T>;
    fn matches_first_char(char: char) -> bool;
}

pub trait ConundrumParserWithParam<T, R> {
    fn parse_input_string(input: &mut ConundrumInput, data: T) -> ConundrumModalResult<R>;
}

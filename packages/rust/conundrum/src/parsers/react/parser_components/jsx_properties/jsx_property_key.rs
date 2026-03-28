use winnow::{
    ModalResult, Parser,
    ascii::alphanumeric1,
    stream::{AsChar, Stream},
    token::take_while,
};

use crate::lang::runtime::traits::conundrum_input::ConundrumInput;

pub fn jsx_property_key(input: &mut ConundrumInput) -> ModalResult<String> {
    let start = input.input.checkpoint();
    let first_char = take_while(1, AsChar::is_alpha).parse_next(input).inspect_err(|_| {
                                                                           input.input.reset(&start);
                                                                       })?;
    let rest_of_key = alphanumeric1.parse_next(input).inspect_err(|_| {
                                                          input.input.reset(&start);
                                                      })?;

    Ok(format!("{}{}", first_char, rest_of_key))
}

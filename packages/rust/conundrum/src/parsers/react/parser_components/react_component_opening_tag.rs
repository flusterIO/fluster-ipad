use winnow::{
    Parser,
    ascii::alphanumeric1,
    stream::{AsChar, Stream},
    token::take_while,
};

use crate::lang::runtime::{state::conundrum_error_variant::ConundrumResult, traits::conundrum_input::ConundrumInput};

pub fn parse_close_of_opening_tag(input: &mut ConundrumInput) -> ConundrumResult<()> {
    let mut in_quotes = false;
    let mut in_object = false;
    // Iterate over the characters one by one here, handling the opening and closing
    // of quotes, objects and other content that would allow the '>' character.
    Ok(())
}

pub fn react_component_opening_tag(input: &mut ConundrumInput) -> ConundrumResult<()> {
    let start = input.input.checkpoint();

    let _ = '<'.parse_next(input).inspect_err(|_| {
                                      input.input.reset(&start);
                                  })?;

    let capital_leading_letter = take_while(1, AsChar::is_alpha).parse_next(input).inspect_err(|_| {
                                                                                       input.input.reset(&start);
                                                                                   })?;

    let rest_of_comp_name = alphanumeric1.parse_next(input).inspect_err(|_| {
                                                                input.input.reset(&start);
                                                            })?;

    let component_name = format!("{}{}", capital_leading_letter, rest_of_comp_name);

    let closing_stuff = parse_close_of_opening_tag.parse_next(input).inspect_err(|_| {
                                                                         input.input.reset(&start);
                                                                     })?;

    Ok(())
}

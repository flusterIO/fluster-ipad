use winnow::{ModalResult, Parser, combinator::delimited, token::take_till};

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::react::parser_components::jsx_properties::jsx_property_key::jsx_property_key,
};

/// Returns the object key and the bracketed content in a tuple, in that order.
pub fn jsx_curly_bracket_wrapped_property(input: &mut ConundrumInput) -> ModalResult<(String, String)> {
    let (key, _, bracketed_content) =
        (jsx_property_key, '=', delimited('{', take_till(0.., |c| c == '}'), '}')).parse_next(input)?;
    Ok((key, bracketed_content.trim().to_string()))
}

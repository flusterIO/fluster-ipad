use winnow::{Parser, combinator::delimited, stream::Stream, token::take_till};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{state::conundrum_error_variant::ConundrumResult, traits::conundrum_input::ConundrumInput},
    },
    parsers::{
        javascript::object::{
            javascript_key_value_pair::JavascriptObjectKeyValuePair, javascript_object_value::javascript_object_value,
        },
        parser_components::consume_white_space::consume_white_space,
        react::parser_components::jsx_properties::jsx_property_key::jsx_property_key,
    },
};

/// Returns the object key and the bracketed content in a tuple, in that order.
pub fn jsx_curly_bracket_wrapped_property(input: &mut ConundrumInput) -> ConundrumResult<(String, String)> {
    let (key, _, bracketed_content) =
        (jsx_property_key, '=', delimited('{', take_till(0.., |c| c == '}'), '}')).parse_next(input)?;
    Ok((key, bracketed_content.trim().to_string()))
}

// RESUME: THis is the error. Come back here and fis this so the object parser
// works after fixing website's fucing math error.
pub fn jsx_curly_bracket_property<T>(mut parser: impl Fn(&mut ConundrumInput) -> ConundrumResult<T>)
                                     -> impl FnMut(&mut ConundrumInput) -> ConundrumResult<(String, T)> {
    move |input| {
        let start = input.input.checkpoint();
        let key = jsx_property_key.parse_next(input).inspect_err(|_| {
                                                         input.input.reset(&start);
                                                     })?;
        '='.void().parse_next(input).inspect_err(|_| {
                                         input.input.reset(&start);
                                     })?;
        '{'.void().parse_next(input).inspect_err(|_| {
                                         input.input.reset(&start);
                                     })?;
        consume_white_space(0..).parse_next(input).inspect_err(|_| {
                                                       input.input.reset(&start);
                                                   })?;
        let t = parser.parse_next(input).inspect_err(|_| {
                                             input.input.reset(&start);
                                         })?;

        consume_white_space(0..).parse_next(input).inspect_err(|_| {
                                                       input.input.reset(&start);
                                                   })?;
        '}'.void().parse_next(input).inspect_err(|_| {
                                         input.input.reset(&start);
                                     })?;
        Ok((key, t))
    }
}

pub fn any_curly_bracket_jsx_property(input: &mut ConundrumInput) -> ConundrumResult<JavascriptObjectKeyValuePair> {
    let (key, em) = jsx_curly_bracket_property(javascript_object_value).parse_next(input)?;

    Ok(JavascriptObjectKeyValuePair { key,
                                      value: Box::new(ParsedElement::Javascript(em)) })
}

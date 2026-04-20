use std::sync::Arc;

use serde::Serialize;
use winnow::{Parser, combinator::alt, stream::Stream, token::literal};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            state::conundrum_error_variant::ConundrumModalResult,
            traits::conundrum_input::{ConundrumInput, get_conundrum_input},
        },
    },
    parsers::{
        conundrum::logic::{string::conundrum_string::ConundrumString, token::ConundrumLogicToken},
        javascript::{
            javascript_parser_trait::JavascriptParser, object::javascript_key_value_pair::JavascriptObjectKeyValuePair,
        },
        react::parser_components::jsx_properties::{
            jsx_curly_bracket_wrapped_property::jsx_curly_bracket_wrapped_property, jsx_property::JsxPropertyParser,
            jsx_property_key::jsx_property_key,
        },
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Default, Clone)]
pub struct JsxStringPropertyResult {}

fn curly_bracket_wrapped_jsx_string_value(input: &mut ConundrumInput)
                                          -> ConundrumModalResult<JavascriptObjectKeyValuePair> {
    let start = input.input.checkpoint();
    let (key, wrapped_content) = jsx_curly_bracket_wrapped_property.parse_next(input).inspect_err(|_| {
                                                                                          input.input.reset(&start);
                                                                                      })?;

    let mut wrapped_content_input = ConundrumInput { input: &wrapped_content,
                                                     state: Arc::clone(&input.state) };

    let js_string = ConundrumString::parse_javascript.parse_next(&mut wrapped_content_input)
                                                     .inspect_err(|_| {
                                                         input.input.reset(&start);
                                                     })?;

    Ok(JavascriptObjectKeyValuePair { key,
                                      value: Box::new(ParsedElement::Logic(ConundrumLogicToken::String(js_string))) })
}

fn not_curly_bracket_wrapped_jsx_string_value(input: &mut ConundrumInput)
                                              -> ConundrumModalResult<JavascriptObjectKeyValuePair> {
    let start = input.input.checkpoint();
    let key = jsx_property_key.parse_next(input).inspect_err(|_| {
                                                     input.input.reset(&start);
                                                 })?;
    let _ = literal("=").parse_next(input).inspect_err(|_| {
                                               input.input.reset(&start);
                                           })?;
    let js_string = alt((
            ConundrumString::single_quoted_javascript_string,
            ConundrumString::double_quoted_javascript_string,
            // TODO: Implement a special backtick syntax without the curly brackets since that's
            // not allowed in jsx anyways that automatically parses the output as Conundrum logic
            // and returns the output like a jupyter cell once the memory layer is in place.
    )).parse_next(input).inspect_err(|_| {
        input.input.reset(&start);
    })?;

    Ok(JavascriptObjectKeyValuePair { key,
                                      value: Box::new(ParsedElement::Logic(ConundrumLogicToken::String(js_string))) })
}

impl JsxPropertyParser for JsxStringPropertyResult {
    fn parse_jsx_property(input: &mut ConundrumInput) -> ConundrumModalResult<JavascriptObjectKeyValuePair> {
        alt((curly_bracket_wrapped_jsx_string_value, not_curly_bracket_wrapped_jsx_string_value)).parse_next(input)
    }
}

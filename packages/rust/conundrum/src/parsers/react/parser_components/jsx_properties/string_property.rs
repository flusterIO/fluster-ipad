use winnow::{ModalResult, Parser, combinator::alt, stream::Stream, token::literal};

use crate::{
    lang::runtime::traits::conundrum_input::{ConundrumInput, get_conundrum_input},
    parsers::{
        javascript::{
            javascript_parser_trait::JavascriptParser,
            string::javascript_string::{self, double_quoted_javascript_string, single_quoted_javascript_string},
        },
        react::parser_components::jsx_properties::{
            jsx_curly_bracket_wrapped_property::jsx_curly_bracket_wrapped_property, jsx_property::JsxPropertyParser,
            jsx_property_key::jsx_property_key,
        },
    },
};

pub struct JsxStringPropertyResult {
    pub key: String,
    pub value: String,
}

fn curly_bracket_wrapped_jsx_string_value(input: &mut ConundrumInput) -> ModalResult<JsxStringPropertyResult> {
    let start = input.input.checkpoint();
    let (key, wrapped_content) = jsx_curly_bracket_wrapped_property.parse_next(input).inspect_err(|_| {
                                                                                          input.input.reset(&start);
                                                                                      })?;

    let state = input.state.borrow();

    let mut wrapped_content_input = get_conundrum_input(&wrapped_content, state.modifiers.clone());

    let js_string = javascript_string::JavascriptStringResult::parse_javascript.parse_next(&mut wrapped_content_input)
                                                                               .inspect_err(|_| {
                                                                                   input.input.reset(&start);
                                                                               })?;

    Ok(JsxStringPropertyResult { key,
                                 value: js_string.value })
}

fn not_curly_bracket_wrapped_jsx_string_value(input: &mut ConundrumInput) -> ModalResult<JsxStringPropertyResult> {
    let start = input.input.checkpoint();
    let key = jsx_property_key.parse_next(input).inspect_err(|_| {
                                                     input.input.reset(&start);
                                                 })?;
    let _ = literal("=").parse_next(input).inspect_err(|_| {
                                               input.input.reset(&start);
                                           })?;
    let s = alt((
            single_quoted_javascript_string,
            double_quoted_javascript_string,
            // TODO: Implement a special backtick syntax without the curly brackets since that's
            // not allowed in jsx anyways that automatically parses the output as Conundrum logic
            // and returns the output like a jupyter cell once the memory layer is in place.
    )).parse_next(input).inspect_err(|_| {
        input.input.reset(&start);
    })?;

    Ok(JsxStringPropertyResult { key,
                                 value: s.value })
}

impl JsxPropertyParser<JsxStringPropertyResult> for JsxStringPropertyResult {
    fn parse_jsx_property(input: &mut ConundrumInput) -> ModalResult<JsxStringPropertyResult> {
        alt((curly_bracket_wrapped_jsx_string_value, not_curly_bracket_wrapped_jsx_string_value)).parse_next(input)
    }
}

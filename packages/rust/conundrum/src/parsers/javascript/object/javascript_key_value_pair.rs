use serde::Serialize;
use winnow::{ModalResult, Parser, stream::Stream};

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::{
        javascript::{
            javascript_parser_trait::JavascriptParser,
            object::{javascript_object_key::javascript_object_key, javascript_object_value::javascript_object_value},
            parsed_javascript_elements::ParsedJavascriptElement,
        },
        parser_components::consume_white_space::consume_white_space,
    },
};

#[derive(Debug, Serialize)]
pub struct JavascriptObjectKeyValuePair {
    pub key: String,
    // Box required to break recursive loop thing, but not sure if this is the best approach.
    // There are other possibilities too...
    pub value: Box<ParsedJavascriptElement>,
}

impl JavascriptParser<JavascriptObjectKeyValuePair> for JavascriptObjectKeyValuePair {
    fn parse_javascript(input: &mut ConundrumInput) -> ModalResult<JavascriptObjectKeyValuePair> {
        let start = input.input.checkpoint();

        let _ = consume_white_space(0..).parse_next(input).inspect_err(|_| {
                                                               input.input.reset(&start);
                                                           })?;
        let key = javascript_object_key.parse_next(input).inspect_err(|_| {
                                                              input.input.reset(&start);
                                                          })?;
        let _ = consume_white_space(0..).parse_next(input).inspect_err(|_| {
                                                               input.input.reset(&start);
                                                           })?;
        let _ = ':'.parse_next(input).inspect_err(|_| {
                                          input.input.reset(&start);
                                      })?;

        let value = javascript_object_value.parse_next(input).inspect_err(|_| {
                                                                  input.input.reset(&start);
                                                              })?;

        Ok(JavascriptObjectKeyValuePair { key,
                                          value: Box::new(value) })
    }
}

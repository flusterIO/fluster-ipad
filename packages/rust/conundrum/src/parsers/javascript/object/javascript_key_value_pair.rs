use serde::Serialize;
use winnow::{Parser, stream::Stream};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            state::conundrum_error_variant::ConundrumResult,
            traits::{conundrum_input::ConundrumInput, fluster_component_result::ConundrumComponentResult},
        },
    },
    parsers::{
        javascript::{
            javascript_parser_trait::JavascriptParser,
            object::{javascript_object_key::javascript_object_key, javascript_object_value::javascript_object_value},
        },
        parser_components::consume_white_space::consume_white_space,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct JavascriptObjectKeyValuePair {
    pub key: String,
    // Box required to break recursive loop thing, but not sure if this is the best approach.
    // There are other possibilities too...
    pub value: Box<ParsedElement>,
}

impl JavascriptParser<JavascriptObjectKeyValuePair> for JavascriptObjectKeyValuePair {
    fn parse_javascript(input: &mut ConundrumInput) -> ConundrumResult<JavascriptObjectKeyValuePair> {
        let start = input.input.checkpoint();
        let key = javascript_object_key.parse_next(input).inspect_err(|_| {
                                                              input.input.reset(&start);
                                                          })?;
        consume_white_space(0..).parse_next(input).inspect_err(|_| {
                                                       input.input.reset(&start);
                                                   })?;
        let _ = ':'.parse_next(input).inspect_err(|_| {
                                          input.input.reset(&start);
                                      })?;

        consume_white_space(0..).parse_next(input).inspect_err(|_| {
                                                       input.input.reset(&start);
                                                   })?;
        let value = javascript_object_value.parse_next(input).inspect_err(|_| {
                                                                  input.input.reset(&start);
                                                              })?;

        Ok(JavascriptObjectKeyValuePair { key,
                                          value: Box::new(ParsedElement::Javascript(value)) })
    }
}

impl ConundrumComponentResult for JavascriptObjectKeyValuePair {
    fn to_conundrum_component(&self, _: &mut crate::lang::runtime::state::parse_state::ParseState) -> String {
        // This element represents nothing visual, only a temporary data representation
        // of jsx kv pairs.
        String::from("")
    }
}

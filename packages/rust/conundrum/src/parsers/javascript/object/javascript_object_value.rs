use winnow::combinator::alt;
use winnow::{ModalResult, Parser};

use crate::lang::runtime::state::conundrum_error_variant::ConundrumResult;
use crate::parsers::javascript::function::javascript_function::JavascriptFunction;
use crate::parsers::javascript::javascript_boolean::JavascriptBooleanResult;
use crate::parsers::javascript::object::javascript_object::JavascriptObjectResult;
use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::javascript::{
        javascript_number::JavascriptNumberResult, javascript_parser_trait::JavascriptParser,
        parsed_javascript_elements::ParsedJavascriptElement, string::javascript_string::JavascriptStringResult,
    },
};

pub fn javascript_object_value(input: &mut ConundrumInput) -> ConundrumResult<ParsedJavascriptElement> {
    alt((JavascriptStringResult::parse_javascript.map(ParsedJavascriptElement::String),
         JavascriptFunction::parse_javascript.map(ParsedJavascriptElement::Function),
         JavascriptObjectResult::parse_javascript.map(ParsedJavascriptElement::Object),
         JavascriptNumberResult::parse_javascript.map(ParsedJavascriptElement::Number),
         JavascriptBooleanResult::parse_javascript.map(ParsedJavascriptElement::Boolean))).parse_next(input)
}

use winnow::combinator::alt;
use winnow::{ModalResult, Parser};

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::javascript::{
        javascript_boolean::JavascriptBooleanResult, javascript_number::JavascriptNumberResult,
        javascript_parser_trait::JavascriptParser, parsed_javascript_elements::ParsedJavascriptElement,
        string::javascript_string::JavascriptStringResult,
    },
};

pub fn javascript_function_parameter(input: &mut ConundrumInput) -> ModalResult<ParsedJavascriptElement> {
    alt((JavascriptBooleanResult::parse_javascript.map(ParsedJavascriptElement::Boolean),
         JavascriptStringResult::parse_javascript.map(ParsedJavascriptElement::String),
         JavascriptNumberResult::parse_javascript.map(ParsedJavascriptElement::Number))).parse_next(input)
}

use winnow::combinator::alt;
use winnow::{ModalResult, Parser};

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::javascript::{
        function::{
            javascript_arrow_function::javascript_arrow_function, javascript_named_function::javascript_function,
        },
        javascript_parser_trait::JavascriptParser,
        parsed_javascript_elements::ParsedJavascriptElement,
    },
};

pub struct JavascriptFunction {
    pub parameters: Vec<ParsedJavascriptElement>,
    pub javascript_body: String,
}

impl JavascriptParser<JavascriptFunction> for JavascriptFunction {
    fn parse_javascript(input: &mut ConundrumInput) -> ModalResult<JavascriptFunction> {
        alt((javascript_function, javascript_arrow_function)).parse_next(input)
    }
}

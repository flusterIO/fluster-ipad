use serde::Serialize;
use winnow::combinator::alt;
use winnow::{ModalResult, Parser};

use crate::lang::runtime::traits::fluster_component_result::ConundrumComponentResult;
use crate::lang::runtime::traits::jsx_component_result::JsxComponentResult;
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

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct JavascriptFunction {
    pub parameters: Vec<ParsedJavascriptElement>,
    pub javascript_body: String,
}

impl JavascriptParser<JavascriptFunction> for JavascriptFunction {
    fn parse_javascript(input: &mut ConundrumInput) -> ModalResult<JavascriptFunction> {
        alt((javascript_function, javascript_arrow_function)).parse_next(input)
    }
}


impl JsxComponentResult for JavascriptFunction {
    fn to_jsx_component(&self, res: &mut crate::lang::runtime::state::parse_state::ParseState) -> String {
        format!("")
    }
}


impl ConundrumComponentResult for JavascriptFunction {
    fn to_conundrum_component(&self, res: &mut crate::lang::runtime::state::parse_state::ParseState) -> String {
        todo!()
    }
}

use serde::Serialize;
use winnow::Parser;
use winnow::combinator::alt;

use crate::lang::runtime::mem::mem::MemoryArc;
use crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult;
use crate::lang::runtime::traits::conundrum_input::ArcState;
use crate::lang::runtime::traits::fluster_component_result::ConundrumComponentResult;
use crate::lang::runtime::traits::jsx_component_result::JsxComponentResult;
use crate::parsers::conundrum::conundrum_logic_parser::ConundrumLogicParser;
use crate::parsers::conundrum::logic::function::inline_conundrum_function_parser::conundrum_inline_function;
use crate::parsers::conundrum::logic::function::named_conundrum_function_parser::conundrum_named_function;
use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::javascript::parsed_javascript_elements::ParsedJavascriptElement,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct ConundrumFunction {
    pub parameters: Vec<ParsedJavascriptElement>,
    pub javascript_body: String,
}

impl ConundrumLogicParser for ConundrumFunction {
    fn parse_conundrum(input: &mut ConundrumInput) -> ConundrumModalResult<Self>
        where Self: Sized {
        alt((conundrum_named_function, conundrum_inline_function)).parse_next(input)
    }
}

impl JsxComponentResult for ConundrumFunction {
    fn to_jsx_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        // FIXME: This obviously won't work. You'll need to handle this all with
        // Conundrum and code blocks and just bail on the javascript environment
        // support.
        Ok(String::from(""))
    }
}

impl ConundrumComponentResult for ConundrumFunction {
    fn to_conundrum_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        todo!()
    }
}

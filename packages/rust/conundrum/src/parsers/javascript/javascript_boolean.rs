use serde::Serialize;
use winnow::Parser;
use winnow::combinator::alt;
use winnow::token::literal;

use crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult;
use crate::lang::runtime::traits::conundrum_input::ConundrumInput;
use crate::lang::runtime::traits::fluster_component_result::ConundrumComponentResult;
use crate::parsers::javascript::javascript_parser_trait::JavascriptParser;

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct JavascriptBooleanResult {
    pub value: bool,
}

impl JavascriptParser<JavascriptBooleanResult> for JavascriptBooleanResult {
    fn parse_javascript(input: &mut ConundrumInput) -> ConundrumModalResult<JavascriptBooleanResult> {
        let res = alt((literal("true"), literal("false"))).parse_next(input)?;
        Ok(JavascriptBooleanResult { value: res == "true" })
    }
}

impl ConundrumComponentResult for JavascriptBooleanResult {
    fn to_conundrum_component(&self,
                              _: &mut crate::lang::runtime::state::parse_state::ParseState)
                              -> ConundrumModalResult<String> {
        if self.value {
            Ok(String::from("true"))
        } else {
            Ok(String::from("false"))
        }
    }
}

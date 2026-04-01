use serde::Serialize;
use winnow::combinator::alt;
use winnow::token::literal;
use winnow::{ModalResult, Parser};

use crate::lang::runtime::traits::conundrum_input::ConundrumInput;
use crate::parsers::javascript::javascript_parser_trait::JavascriptParser;

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct JavascriptBooleanResult {
    pub value: bool,
}

impl JavascriptParser<JavascriptBooleanResult> for JavascriptBooleanResult {
    fn parse_javascript(input: &mut ConundrumInput) -> ModalResult<JavascriptBooleanResult> {
        let res = alt((literal("true"), literal("false"))).parse_next(input)?;
        Ok(JavascriptBooleanResult { value: res == "true" })
    }
}

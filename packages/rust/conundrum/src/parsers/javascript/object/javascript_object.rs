use winnow::{
    ModalResult, Parser,
    combinator::{delimited, separated},
};

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::{
        javascript::{
            javascript_parser_trait::JavascriptParser, object::javascript_key_value_pair::JavascriptObjectKeyValuePair,
        },
        parser_components::white_space_delimited::white_space_delimited,
    },
};

pub struct JavascriptObjectResult {
    pub entries: Vec<JavascriptObjectKeyValuePair>,
}

impl JavascriptParser<JavascriptObjectResult> for JavascriptObjectResult {
    fn parse_javascript(input: &mut ConundrumInput) -> ModalResult<JavascriptObjectResult> {
        let entries: Vec<JavascriptObjectKeyValuePair> =
            delimited('{',
                      separated(0.., white_space_delimited(JavascriptObjectKeyValuePair::parse_javascript), ','),
                      '}').parse_next(input)?;

        Ok(JavascriptObjectResult { entries })
    }
}

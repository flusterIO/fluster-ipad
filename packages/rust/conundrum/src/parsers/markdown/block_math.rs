use serde::Serialize;
use winnow::{
    ModalResult, Parser,
    combinator::delimited,
    token::{literal, take_until},
};

use crate::{
    lang::runtime::{
        state::parse_state::{ConundrumModifier, ParseState},
        traits::{
            conundrum_input::ConundrumInput, fluster_component_result::FlusterComponentResult,
            mdx_component_result::MdxComponentResult, plain_text_component_result::PlainTextComponentResult,
        },
    },
    parsers::parser_trait::ConundrumParser,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize)]
pub struct BlockMathResult {
    pub body: String,
}

impl PlainTextComponentResult for BlockMathResult {
    fn to_plain_text(&self, _: &mut ParseState) -> String {
        self.body.clone()
    }
}

impl FlusterComponentResult for BlockMathResult {
    fn to_fluster_component(&self, res: &mut ParseState) -> String {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MdxComponentResult for BlockMathResult {
    fn to_mdx_component(&self, _: &mut ParseState) -> String {
        format!("$$\n{}\n$$", self.body)
    }
}

impl ConundrumParser<BlockMathResult> for BlockMathResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ModalResult<BlockMathResult> {
        let body = delimited(literal("$$"), take_until(1.., "$$"), literal("$$")).parse_next(input)?;
        Ok(BlockMathResult { body: body.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '$'
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_block_math_content() {
        let test_content = "$$\n\ne=mc^2\n\n$$";
        let mut test_props = wrap_test_conundrum_content(test_content);
        let res =
            BlockMathResult::parse_input_string(&mut test_props).expect("Parses math block without throwing an error.");
        assert_eq!(res.body, "\n\ne=mc^2\n\n", "Finds the proper math body when parsing inline math.");
    }
}

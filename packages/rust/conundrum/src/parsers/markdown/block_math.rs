use serde::Serialize;
use winnow::{
    ModalResult, Parser,
    combinator::delimited,
    token::{literal, take_until},
};

use crate::{
    lang::runtime::traits::{conundrum_input::ConundrumInput, mdx_component_result::MdxComponentResult},
    parsers::parser_trait::ConundrumParser,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize)]
pub struct BlockMathResult {
    pub body: String,
}

impl MdxComponentResult for BlockMathResult {
    fn to_mdx_component(&self, _: &mut crate::output::parsing_result::mdx_parsing_result::MdxParsingResult) -> String {
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

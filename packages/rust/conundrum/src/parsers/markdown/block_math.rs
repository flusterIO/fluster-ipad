use serde::Serialize;
use winnow::{
    Parser,
    combinator::delimited,
    token::{literal, take_until},
};

use crate::{
    lang::runtime::{
        state::{
            conundrum_error_variant::{ConundrumModalResult, ConundrumResult},
            parse_state::{ConundrumModifier, ParseState},
        },
        traits::{
            conundrum_input::ConundrumInput, fluster_component_result::ConundrumComponentResult,
            markdown_component_result::MarkdownComponentResult, mdx_component_result::MdxComponentResult,
            plain_text_component_result::PlainTextComponentResult,
        },
    },
    parsers::parser_trait::ConundrumParser,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct BlockMathResult {
    pub body: String,
}

impl PlainTextComponentResult for BlockMathResult {
    fn to_plain_text(&self, _: &mut ParseState) -> String {
        self.body.clone()
    }
}

impl ConundrumComponentResult for BlockMathResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> String {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else if res.is_markdown() {
            self.to_markdown(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MarkdownComponentResult for BlockMathResult {
    fn to_markdown(&self, res: &mut ParseState) -> String {
        format!("$$\n{}\n$$", self.body)
    }
}

impl MdxComponentResult for BlockMathResult {
    fn to_mdx_component(&self, _: &mut ParseState) -> String {
        format!("<div className=\"conundrum-math conundrum-math-block\">\n$${}$$\n</div>", self.body)
    }
}

impl ConundrumParser<BlockMathResult> for BlockMathResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<BlockMathResult> {
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

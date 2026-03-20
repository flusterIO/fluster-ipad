use fluster_core_utilities::core_types::component_constants::auto_inserted_component_name::AutoInsertedComponentName;
use winnow::{
    ModalResult, Parser,
    ascii::take_escaped,
    combinator::{delimited, opt},
    token::{literal, one_of, take_till},
};

use crate::{
    lang::runtime::traits::{conundrum_input::ConundrumInput, mdx_component_result::MdxComponentResult},
    parsers::parser_trait::ConundrumParser,
};

#[derive(Debug)]
pub struct BlockMathResult {
    pub body: String,
}

impl MdxComponentResult for BlockMathResult {
    fn to_mdx_component(&self, _: &mut crate::output::parsing_result::mdx_parsing_result::MdxParsingResult) -> String {
        format!("\n<{}>\n{}\n</{}>\n",
                AutoInsertedComponentName::AutoInsertedBlockMath,
                self.body,
                AutoInsertedComponentName::AutoInsertedBlockMath,)
    }
}

impl ConundrumParser<BlockMathResult> for BlockMathResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ModalResult<BlockMathResult> {
        let body = delimited(literal("$$"),
                             opt(take_escaped(// 1. "Normal" text: Must consume AT LEAST 1 char, and stop at \ or $
                                              take_till(1.., ('\\', '$')),
                                              // 2. Control character: The escape trigger
                                              '\\',
                                              // 3. Escapable: What is legally allowed to follow the \
                                              one_of(['$', '\\']))).map(|s| s.unwrap_or("")),
                             literal("$$")).parse_next(input)?;
        Ok(BlockMathResult { body: body.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '$'
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::{self, wrap_test_conundrum_content};

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

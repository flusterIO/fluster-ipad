use serde::Serialize;
use winnow::{
    ModalResult, Parser,
    combinator::{alt, delimited},
    token::take_while,
};

use crate::{
    lang::runtime::traits::{conundrum_input::ConundrumInput, mdx_component_result::MdxComponentResult},
    parsers::parser_trait::ConundrumParser,
};

#[derive(Debug, Serialize)]
pub struct MarkdownItalicTextResult {
    pub content: String,
}

impl MdxComponentResult for MarkdownItalicTextResult {
    fn to_mdx_component(&self, _: &mut crate::output::parsing_result::mdx_parsing_result::MdxParsingResult) -> String {
        format!("<span className=\"italic\">{}</span>", self.content)
    }
}

impl ConundrumParser<MarkdownItalicTextResult> for MarkdownItalicTextResult {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ModalResult<MarkdownItalicTextResult> {
        let content = alt((delimited('*', take_while(1.., |c| c != '\n' && c != '*'), '*'),
                           delimited('_', take_while(1.., |c| c != '\n' && c != '_'), '_'))).parse_next(input)?;

        Ok(MarkdownItalicTextResult { content: content.to_string() })
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
    fn markdown_italic_text_asterisk() {
        let test_input = "*My italic text*";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownItalicTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        assert!(res.content == "My italic text", "Finds the proper text in the markdown italic text with asterisks.");
        // assert_eq!(result, 4);
    }

    #[test]
    fn markdown_italic_text_underscores() {
        let test_input = "_My italic text_";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownItalicTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        assert!(res.content == "My italic text", "Finds the proper text in the markdown italic text with underscors.");
        // assert_eq!(result, 4);
    }
}

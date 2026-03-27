use serde::Serialize;
use winnow::{
    ModalResult, Parser,
    combinator::{alt, delimited},
    token::take_while,
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

#[derive(Debug, Serialize)]
pub struct MarkdownItalicTextResult {
    pub content: String,
}

impl PlainTextComponentResult for MarkdownItalicTextResult {
    fn to_plain_text(&self, _: &mut ParseState) -> String {
        self.content.clone()
    }
}

impl MdxComponentResult for MarkdownItalicTextResult {
    fn to_mdx_component(&self, _: &mut ParseState) -> String {
        format!("<span className=\"italic\">{}</span>", self.content)
    }
}

impl FlusterComponentResult for MarkdownItalicTextResult {
    fn to_fluster_component(&self, res: &mut ParseState) -> String {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
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

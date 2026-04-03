use serde::Serialize;
use winnow::{
    Parser,
    combinator::{alt, delimited},
    token::take_while,
};

use crate::{
    lang::runtime::{
        state::{
            conundrum_error_variant::ConundrumResult,
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
pub struct MarkdownItalicTextResult {
    pub content: String,
}

impl MarkdownComponentResult for MarkdownItalicTextResult {
    fn to_markdown(&self, _: &mut ParseState) -> String {
        format!("_{}_", self.content)
    }
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

impl ConundrumComponentResult for MarkdownItalicTextResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> String {
        if res.contains_one_of_modifiers(vec![ConundrumModifier::ForcePlainText, ConundrumModifier::ForSearchInput]) {
            self.to_plain_text(res)
        } else if res.contains_one_of_modifiers(vec![ConundrumModifier::PreferMarkdownSyntax,
                                                     ConundrumModifier::PreferInlineMarkdownSyntax,])
        {
            self.to_markdown(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl ConundrumParser<MarkdownItalicTextResult> for MarkdownItalicTextResult {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ConundrumResult<MarkdownItalicTextResult> {
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

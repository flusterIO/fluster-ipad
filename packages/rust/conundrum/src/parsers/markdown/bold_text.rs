use serde::Serialize;
use winnow::{
    ModalResult, Parser,
    combinator::alt,
    token::{literal, take_while},
};

use crate::{
    lang::runtime::{
        state::parse_state::ParseState,
        traits::{
            conundrum_input::ConundrumInput, mdx_component_result::MdxComponentResult,
            plain_text_component_result::PlainTextComponentResult,
        },
    },
    parsers::parser_trait::ConundrumParser,
};

#[derive(Debug, Serialize)]
pub struct MarkdownBoldTextResult {
    pub content: String,
}

impl PlainTextComponentResult for MarkdownBoldTextResult {
    fn to_plain_text(&self, _: &mut ParseState) -> String {
        self.content.clone()
    }
}

impl MdxComponentResult for MarkdownBoldTextResult {
    fn to_mdx_component(&self, _: &mut ParseState) -> String {
        format!("<span className=\"font-bold\">{}</span>", self.content)
    }
}

impl ConundrumParser<MarkdownBoldTextResult> for MarkdownBoldTextResult {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ModalResult<MarkdownBoldTextResult> {
        // FIXME: Handle the resetting of state to this checkpoint if the entire parser
        // fails. let cp = input.checkpoint();
        let first_token = alt((literal("*"), literal("_"))).parse_next(input)?;
        let second_token = alt((literal("*"), literal("_"))).parse_next(input)?;
        let content = take_while(1.., |c: char| c.to_string() != second_token && c != '\n').parse_next(input)?;
        let _ = literal(second_token).parse_next(input)?;
        let _ = literal(first_token).parse_next(input)?;

        Ok(MarkdownBoldTextResult { content: content.to_string() })
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
    fn markdown_bold_text_asterisk() {
        let test_input = "**My bold text**";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        assert!(res.content == "My bold text", "Finds the proper text in the markdown bold text with asterisks.");
    }

    #[test]
    fn markdown_bold_text_mixed_brackets() {
        let test_input = "_*My bold text*_";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        assert!(res.content == "My bold text", "Finds the proper text in the markdown bold text with underscors.");
    }

    #[test]
    fn markdown_bold_text_underscores() {
        let test_input = "__My bold text__";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        assert!(res.content == "My bold text", "Finds the proper text in the markdown bold text with underscors.");
    }

    #[test]
    fn markdown_bold_text_returns_complete_text_on_fail() {
        let test_input = "*Some other text that will fail.";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldTextResult::parse_input_string(&mut wrapped);

        println!("Wrapped: {}", wrapped.input);

        assert!(res.is_err(), "Returns an error when parser fails.");

        assert!(wrapped.input == "*Some other text that will fail.",
                "Bold text parser returns the complete text when the parser fails.")
    }
}

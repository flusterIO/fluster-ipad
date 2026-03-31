use serde::Serialize;
use winnow::{
    ModalResult, Parser,
    combinator::alt,
    stream::Stream,
    token::{literal, take_while},
};

use crate::{
    lang::runtime::{
        state::parse_state::{ConundrumModifier, ParseState},
        traits::{
            conundrum_input::ConundrumInput, fluster_component_result::ConundrumComponentResult,
            markdown_component_result::MarkdownComponentResult, mdx_component_result::MdxComponentResult,
            plain_text_component_result::PlainTextComponentResult,
        },
    },
    parsers::parser_trait::ConundrumParser,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize)]
pub struct MarkdownBoldTextResult {
    pub content: String,
}

impl MarkdownComponentResult for MarkdownBoldTextResult {
    fn to_markdown(&self, _: &mut ParseState) -> String {
        format!("**{}**", self.content)
    }
}

impl MdxComponentResult for MarkdownBoldTextResult {
    fn to_mdx_component(&self, _: &mut ParseState) -> String {
        format!("<span className=\"font-bold\">{}</span>", self.content)
    }
}

impl PlainTextComponentResult for MarkdownBoldTextResult {
    fn to_plain_text(&self, _: &mut ParseState) -> String {
        self.content.clone()
    }
}

impl ConundrumComponentResult for MarkdownBoldTextResult {
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

impl ConundrumParser<MarkdownBoldTextResult> for MarkdownBoldTextResult {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ModalResult<MarkdownBoldTextResult> {
        // FIXME: Handle the resetting of state to this checkpoint if the entire parser
        // fails.
        let start = input.input.checkpoint();
        let first_token = alt((literal("*"), literal("_"))).parse_next(input).inspect_err(|_| {
                                                                                  input.input.reset(&start);
                                                                              })?;
        let second_token = alt((literal("*"), literal("_"))).parse_next(input).inspect_err(|_| {
                                                                                   input.input.reset(&start);
                                                                               })?;
        let content = take_while(1.., |c: char| c.to_string() != second_token && c != '\n').parse_next(input)
                                                                                           .inspect_err(|_| {
                                                                                               input.input
                                                                                                    .reset(&start);
                                                                                           })?;
        let _ = literal(second_token).parse_next(input).inspect_err(|_| {
                                                            input.input.reset(&start);
                                                        })?;
        let _ = literal(first_token).parse_next(input).inspect_err(|_| {
                                                           input.input.reset(&start);
                                                       })?;

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

        assert!(res.content == "My bold text", "Finds the proper text in the markdown bold text with underscores.");
    }

    #[test]
    fn markdown_bold_text_underscores() {
        let test_input = "__My bold text__";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        assert!(res.content == "My bold text", "Finds the proper text in the markdown bold text with underscores.");
    }

    #[test]
    fn returns_complete_text_on_fail() {
        let test_input = "*Some other text that will fail.";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldTextResult::parse_input_string(&mut wrapped);

        assert!(res.is_err(), "Returns an error when parser fails.");

        assert!(wrapped.input == "*Some other text that will fail.",
                "Bold text parser returns the complete text when the parser fails.")
    }
}

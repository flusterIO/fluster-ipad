use serde::Serialize;
use winnow::{
    Parser,
    combinator::alt,
    stream::Stream,
    token::{literal, take_while},
};

use crate::{
    lang::runtime::{
        state::{
            conundrum_error_variant::ConundrumModalResult,
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
pub struct MarkdownBoldAndItalicTextResult {
    pub content: String,
}

impl MarkdownComponentResult for MarkdownBoldAndItalicTextResult {
    fn to_markdown(&self, _: &mut ParseState) -> String {
        format!("**_{}_**", self.content)
    }
}

impl MdxComponentResult for MarkdownBoldAndItalicTextResult {
    fn to_mdx_component(&self, _: &mut ParseState) -> String {
        format!("<span className=\"italic font-bold\">{}</span>", self.content)
    }
}

impl PlainTextComponentResult for MarkdownBoldAndItalicTextResult {
    fn to_plain_text(&self, _: &mut ParseState) -> String {
        self.content.clone()
    }
}

impl ConundrumComponentResult for MarkdownBoldAndItalicTextResult {
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

impl ConundrumParser<MarkdownBoldAndItalicTextResult> for MarkdownBoldAndItalicTextResult {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<MarkdownBoldAndItalicTextResult> {
        let cp = input.input.checkpoint();
        let first_token = alt((literal("*"), literal("_"))).parse_next(input).inspect_err(|_| {
                                                                                  input.input.reset(&cp);
                                                                              })?;
        let second_token = alt((literal("*"), literal("_"))).parse_next(input).inspect_err(|_| {
                                                                                   input.input.reset(&cp);
                                                                               })?;
        let third_token = alt((literal("*"), literal("_"))).parse_next(input).inspect_err(|_| {
                                                                                  input.input.reset(&cp);
                                                                              })?;
        let content = take_while(1.., |c: char| c.to_string() != third_token && c != '\n').parse_next(input)
                                                                                          .inspect_err(|e| {
                                                                                              input.input.reset(&cp);
                                                                                          })?;
        let _ = literal(third_token).parse_next(input).inspect_err(|_| {
                                                           input.input.reset(&cp);
                                                       })?;
        let _ = literal(second_token).parse_next(input).inspect_err(|_| {
                                                            input.input.reset(&cp);
                                                        })?;
        let _ = literal(first_token).parse_next(input).inspect_err(|_| {
                                                           input.input.reset(&cp);
                                                       })?;

        Ok(MarkdownBoldAndItalicTextResult { content: content.to_string() })
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
    fn markdown_bold_and_italic_text_asterisk() {
        let test_input = "***My bold and italic text***";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldAndItalicTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        assert!(res.content == "My bold and italic text",
                "Finds the proper text in the markdown bold and italic text with asterisks.");
    }

    #[test]
    fn markdown_bold_and_italic_text_mixed_brackets() {
        let test_input = "__*My bold and italic text*__";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldAndItalicTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        assert!(res.content == "My bold and italic text",
                "Finds the proper text in the markdown bold and italic text with underscores.");
    }

    #[test]
    fn markdown_bold_and_italic_text_underscores() {
        let test_input = "___My bold and italic text___";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldAndItalicTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        assert!(res.content == "My bold and italic text",
                "Finds the proper text in the markdown bold and italic text with underscores.");
    }

    #[test]
    fn markdown_bold_and_italic_text_returns_complete_text_on_fail() {
        let test_input = "*Some other text that will fail.";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldAndItalicTextResult::parse_input_string(&mut wrapped);

        println!("Wrapped: {}", wrapped.input);

        assert!(res.is_err(), "Returns an error when parser fails.");

        assert!(
                wrapped.input == "*Some other text that will fail.",
                "bold and italic text parser returns the complete text when
    the parser fails."
        )
    }
}

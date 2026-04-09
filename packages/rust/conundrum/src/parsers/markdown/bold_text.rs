use serde::Serialize;
use winnow::{
    Parser,
    ascii::newline,
    combinator::{alt, repeat_till},
    stream::Stream,
    token::{any, literal, take},
};

use crate::{
    lang::{
        lib::{shared::traits::from_with_state::FromWithState, ui::ui_types::children::Children},
        runtime::{
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
    },
    parsers::parser_trait::ConundrumParser,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct MarkdownBoldTextResult {
    pub children: Children,
}

impl MarkdownComponentResult for MarkdownBoldTextResult {
    fn to_markdown(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(format!("**{}**", self.children.render(res)?))
    }
}

impl MdxComponentResult for MarkdownBoldTextResult {
    fn to_mdx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(format!("<span className=\"font-bold\">{}</span>", self.children.render(res)?))
    }
}

impl PlainTextComponentResult for MarkdownBoldTextResult {
    fn to_plain_text(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl ConundrumComponentResult for MarkdownBoldTextResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
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
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<MarkdownBoldTextResult> {
        let start = input.input.checkpoint();
        let first_token = alt((literal("*"), literal("_"))).parse_next(input).inspect_err(|_| {
                                                                                  input.input.reset(&start);
                                                                              })?;
        let second_token = alt((literal("*"), literal("_"))).parse_next(input).inspect_err(|_| {
                                                                                   input.input.reset(&start);
                                                                               })?;

        let (c, _): (Vec<&str>, ()) = repeat_till(1..,
                                                  take(1usize),
                                                  alt(((literal(second_token), literal(first_token)).void(),
                                                       newline.void()))).parse_next(input)
                                                                        .inspect_err(|_| {
                                                                            input.input.reset(&start);
                                                                        })?;
        let content = String::from_iter(c);

        let mut state = input.state.borrow_mut();

        let children = Children::from_with_state(content.as_str(), &mut state)?;

        Ok(MarkdownBoldTextResult { children })
    }

    fn matches_first_char(char: char) -> bool {
        char == '_' || char == '*'
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

        let mut state = wrapped.state.borrow_mut();
        assert!(res.children.render(&mut state).is_ok_and(|s| s == "My bold text"),
                "Finds the proper text in the markdown bold text with asterisks.");
    }

    #[test]
    fn markdown_bold_text_mixed_brackets() {
        let test_input = "_*My bold text*_";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        let mut state = wrapped.state.borrow_mut();
        assert!(res.children.render(&mut state).is_ok_and(|s| s == "My bold text"),
                "Finds the proper text in the markdown bold text with underscores.");
    }

    #[test]
    fn markdown_bold_text_underscores() {
        let test_input = "__My bold text__";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        let mut state = wrapped.state.borrow_mut();
        assert!(res.children.render(&mut state).is_ok_and(|s| s == "My bold text"),
                "Finds the proper text in the markdown bold text with underscores.");
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

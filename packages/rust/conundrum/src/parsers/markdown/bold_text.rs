use serde::Serialize;
use std::sync::Arc;
use winnow::{
    Parser,
    ascii::newline,
    combinator::{alt, repeat_till},
    stream::Stream,
    token::{literal, take},
};

use crate::{
    lang::{
        lib::{shared::traits::from_with_state::FromWithState, ui::ui_types::children::Children},
        runtime::{
            state::conundrum_error_variant::ConundrumModalResult,
            traits::{
                conundrum_input::{ArcState, ConundrumInput},
                html_js_component_result::HtmlJsComponentResult,
                markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult,
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

impl HtmlJsComponentResult for MarkdownBoldTextResult {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("<span class=\"font-bold\">{}</span>", self.children.render(res)?))
    }
}

impl MarkdownComponentResult for MarkdownBoldTextResult {
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("**{}**", self.children.render(res)?))
    }
}

impl MdxComponentResult for MarkdownBoldTextResult {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("<span class=\"font-bold\">{}</span>", self.children.render(res)?))
    }
}

impl PlainTextComponentResult for MarkdownBoldTextResult {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.children.render(res)
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

        let children = Children::from_with_state(content.as_str(), Arc::clone(&input.state))?;

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

        assert!(res.children.render(Arc::clone(&wrapped.state)).is_ok_and(|s| s == "My bold text"),
                "Finds the proper text in the markdown bold text with asterisks.");
    }

    #[test]
    fn markdown_bold_text_mixed_brackets() {
        let test_input = "_*My bold text*_";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        assert!(res.children.render(Arc::clone(&wrapped.state)).is_ok_and(|s| s == "My bold text"),
                "Finds the proper text in the markdown bold text with underscores.");
    }

    #[test]
    fn markdown_bold_text_underscores() {
        let test_input = "__My bold text__";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        assert!(res.children.render(Arc::clone(&wrapped.state)).is_ok_and(|s| s == "My bold text"),
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

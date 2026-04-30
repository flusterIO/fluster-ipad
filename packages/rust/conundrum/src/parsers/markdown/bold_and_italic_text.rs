use serde::Serialize;
use std::sync::Arc;
use winnow::{
    Parser,
    combinator::{alt, repeat_till},
    stream::Stream,
    token::{literal, take},
};

use crate::{
    lang::{
        lib::{shared::traits::from_with_state::FromWithState, ui::ui_types::children::Children},
        runtime::{
            state::{conundrum_error_variant::ConundrumModalResult, parse_state::ConundrumModifier},
            traits::{
                conundrum_input::{ArcState, ConundrumInput},
                fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult,
                markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    parsers::{
        markdown::markdown_extensions::markdown_bold_italic_anchor::markdown_bold_italic_anchor,
        parser_trait::ConundrumParser,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct MarkdownBoldAndItalicTextResult {
    pub children: Children,
}

impl MarkdownComponentResult for MarkdownBoldAndItalicTextResult {
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("**_{}_**", self.children.render(res)?))
    }
}

impl MdxComponentResult for MarkdownBoldAndItalicTextResult {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("<span class=\"italic font-bold\">{}</span>", self.children.render(res)?))
    }
}

impl HtmlJsComponentResult for MarkdownBoldAndItalicTextResult {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("<span class=\"italic font-bold\">{}</span>", self.children.render(res)?))
    }
}

impl PlainTextComponentResult for MarkdownBoldAndItalicTextResult {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl ConundrumComponentResult for MarkdownBoldAndItalicTextResult {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.contains_one_of_modifiers(vec![ConundrumModifier::ForSearchInput]) {
            drop(state);
            self.to_plain_text(res)
        } else if state.contains_modifier(&ConundrumModifier::PreferInlineMarkdownSyntax) {
            drop(state);
            self.to_markdown(res)
        } else {
            drop(state);
            self.to_mdx_component(res)
        }
    }
}

impl ConundrumParser<MarkdownBoldAndItalicTextResult> for MarkdownBoldAndItalicTextResult {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<MarkdownBoldAndItalicTextResult> {
        let cp = input.input.checkpoint();
        let first_token = markdown_bold_italic_anchor.parse_next(input).inspect_err(|_| {
                                                                            input.input.reset(&cp);
                                                                        })?;
        let second_token = markdown_bold_italic_anchor.parse_next(input).inspect_err(|_| {
                                                                             input.input.reset(&cp);
                                                                         })?;
        let third_token = markdown_bold_italic_anchor.parse_next(input).inspect_err(|_| {
                                                                            input.input.reset(&cp);
                                                                        })?;

        let (c, _): (Vec<&str>, ()) = repeat_till(1..,
                                                  take(1usize),
                                                  alt(((literal(third_token.as_str()),
                                                        literal(second_token.as_str()),
                                                        literal(first_token.as_str()))
                                                                                      .void(),
                                                       '\n'.void()))).parse_next(input)
                                                                     .inspect_err(|_| {
                                                                         input.input.reset(&cp);
                                                                     })?;
        let content = String::from_iter(c);

        let children = Children::from_with_state(content.as_str(), Arc::clone(&input.state))?;

        Ok(MarkdownBoldAndItalicTextResult { children })
    }

    fn matches_first_char(char: char) -> bool {
        char == '*' || char == '_'
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
        let res = MarkdownBoldAndItalicTextResult::parse_input_string(&mut wrapped).expect("Parses markdown bold & italic text without throwing an error.");

        assert!(res.children.render(Arc::clone(&wrapped.state)).is_ok_and(|x| x == ("My bold and italic text")),
                "Finds the proper text in the markdown bold and italic text with asterisks.");
        assert!(wrapped.input.is_empty(), "Consumes the entire input");
    }

    #[test]
    fn markdown_bold_and_italic_text_mixed_brackets() {
        let test_input = "__*My bold and italic text*__";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldAndItalicTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        assert!(res.children.render(Arc::clone(&wrapped.state)).is_ok_and(|x| x == "My bold and italic text"),
                "Finds the proper text in the markdown bold and italic text with underscores.");
    }

    #[test]
    fn markdown_bold_and_italic_text_underscores() {
        let test_input = "___My bold and italic text___";
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownBoldAndItalicTextResult::parse_input_string(&mut wrapped).expect("Parses markdown link without throwing an error.");

        assert!(res.children.render(Arc::clone(&wrapped.state)).is_ok_and(|x| x == "My bold and italic text"),
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

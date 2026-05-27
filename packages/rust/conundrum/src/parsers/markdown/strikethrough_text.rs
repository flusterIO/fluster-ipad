use serde::{Deserialize, Serialize};
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
    parsers::{parser_trait::ConundrumParser, shared_parser_utils::newline_or_other::NewLineOr},
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarkdownStrikeThroughTextResult {
    pub children: Children,
}

impl HtmlJsComponentResult for MarkdownStrikeThroughTextResult {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let children_string = self.children.render(res)?;
        Ok(format!("<del class=\"strike-through decoration-emphasis-error\">{}</del>", children_string))
    }
}

impl MarkdownComponentResult for MarkdownStrikeThroughTextResult {
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("~~{}~~", self.children.render(res)?))
    }
}

impl PlainTextComponentResult for MarkdownStrikeThroughTextResult {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl MdxComponentResult for MarkdownStrikeThroughTextResult {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(format!("<del className=\"strike-through\">{}</del>", self.children.render(res)?))
    }
}

impl ConundrumParser<MarkdownStrikeThroughTextResult> for MarkdownStrikeThroughTextResult {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<MarkdownStrikeThroughTextResult> {
        let start = input.input.checkpoint();
        literal("~~").parse_next(input).inspect_err(|_| {
                                            input.input.reset(&start);
                                        })?;
        let (content, _): (Vec<&str>, NewLineOr<String>) = repeat_till(1..,
                                                                       take(1usize),
                                                                       alt((newline.map(|_| NewLineOr::Newline),
                                                                            literal("~~").map(|s| {
                                                                                NewLineOr::Other(String::from(s))
                                                                            })))).verify(|(_, terminator)| {
                                                                                     match terminator {
                                                                                         NewLineOr::Newline => false,
                                                                                         NewLineOr::Other(_) => true,
                                                                                     }
                                                                                 })
                                                                                 .parse_next(input)?;

        let children = Children::from_with_state(content.join("").as_str(), Arc::clone(&input.state))?;

        Ok(MarkdownStrikeThroughTextResult { children })
    }

    fn matches_first_char(char: char) -> bool {
        char == '~'
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn markdown_strikethrough_text() {
        let test_input = r#"~~Some strikethrough text~~"#;
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownStrikeThroughTextResult::parse_input_string(&mut wrapped).expect("Parses markdown strike through text without throwing an error.");

        assert!(res.children.render(Arc::clone(&wrapped.state)).is_ok_and(|s| s == "Some strikethrough text"),
                "Finds the proper text in the markdown strikethrough text.");
    }

    #[test]
    fn markdown_strikethrough_text_fails_newline() {
        let test_input = r#"~~Some strikethrough
text~~"#;
        let mut wrapped = wrap_test_conundrum_content(test_input);
        let res = MarkdownStrikeThroughTextResult::parse_input_string(&mut wrapped).expect_err("Parses markdown strike through text without throwing an error.");
    }
}

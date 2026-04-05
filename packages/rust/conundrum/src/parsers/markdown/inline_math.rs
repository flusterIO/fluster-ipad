use serde::Serialize;
use typeshare::typeshare;
use winnow::{Parser, combinator::delimited, token::take_till};

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

#[typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct InlineMathResult {
    pub body: String,
}

impl PlainTextComponentResult for InlineMathResult {
    fn to_plain_text(&self, _: &mut ParseState) -> String {
        self.body.clone()
    }
}

impl ConundrumComponentResult for InlineMathResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> String {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else if res.is_markdown() {
            self.to_markdown(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MarkdownComponentResult for InlineMathResult {
    fn to_markdown(&self, _: &mut ParseState) -> String {
        format!("${}$", self.body)
    }
}

impl MdxComponentResult for InlineMathResult {
    fn to_mdx_component(&self, _: &mut ParseState) -> String {
        format!("<span className=\"conundrum-math conundrum-math-inline\">{}</span>", self.body)
    }
}

impl ConundrumParser<InlineMathResult> for InlineMathResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumResult<InlineMathResult> {
        let body = delimited('$', take_till(1.., |c: char| c == '$'), '$').parse_next(input)?;
        Ok(InlineMathResult { body: body.to_string() })
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
    fn parses_math_content() {
        let test_content = r#"$e=mc^2$"#;
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res =
            InlineMathResult::parse_input_string(&mut test_data).expect("Parses math block without throwing an error.");
        assert_eq!(res.body, "e=mc^2", "Finds the proper math body when parsing inline math.");
    }
}

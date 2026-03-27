use serde::Serialize;
use typeshare::typeshare;
use winnow::{ModalResult, Parser, combinator::delimited, token::take_till};

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

#[typeshare]
#[derive(Debug, Serialize)]
pub struct InlineMathResult {
    pub body: String,
}

impl PlainTextComponentResult for InlineMathResult {
    fn to_plain_text(&self, _: &mut ParseState) -> String {
        self.body.clone()
    }
}

impl FlusterComponentResult for InlineMathResult {
    fn to_fluster_component(&self, res: &mut ParseState) -> String {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MdxComponentResult for InlineMathResult {
    fn to_mdx_component(&self, _: &mut ParseState) -> String {
        format!("${}$", self.body)
    }
}

impl ConundrumParser<InlineMathResult> for InlineMathResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ModalResult<InlineMathResult> {
        let body = delimited('$', take_till(1.., |c: char| c != '$'), '$').parse_next(input)?;
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

use serde::Serialize;
use typeshare::typeshare;
use winnow::{Parser, combinator::delimited, token::take_till};

use crate::{
    lang::{
        lib::ui::components::markdown::math::props::MathData,
        runtime::{
            state::{
                conundrum_error_variant::{ConundrumModalResult, ConundrumResult},
                parse_state::{ConundrumModifier, ParseState},
            },
            traits::{
                conundrum_input::ConundrumInput, fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult, jsx_component_result::JsxComponentResult,
                markdown_component_result::MarkdownComponentResult, mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::auto_inserted_component_name::AutoInsertedComponentName,
    parsers::{conundrum::logic::string::conundrum_string::ConundrumString, parser_trait::ConundrumParser},
};

#[typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct InlineMathResult {
    pub body: ConundrumString,
}

impl PlainTextComponentResult for InlineMathResult {
    fn to_plain_text(&self, _: &mut ParseState) -> String {
        self.body.0.clone()
    }
}

impl JsxComponentResult for InlineMathResult {
    fn to_jsx_component(&self, res: &mut ParseState) -> String {
        format!("<{}>{}</{}>",
                AutoInsertedComponentName::AutoInsertedMathBlock,
                self.body.0.clone(),
                AutoInsertedComponentName::AutoInsertedMathBlock,)
    }
}

impl HtmlJsComponentResult for InlineMathResult {
    fn to_html_js_component(&self, res: &mut ParseState) -> String {
        format!("<span className=\"conundrum-math conundrum-math-inline\">${}$</span>", self.body.0)
    }
}

impl ConundrumComponentResult for InlineMathResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> String {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else if res.is_markdown_or_search_or_ai() {
            self.to_markdown(res)
        } else if res.targets_jsx() {
            self.to_jsx_component(res)
        } else if res.targets_html_js() {
            self.to_html_js_component(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MarkdownComponentResult for InlineMathResult {
    fn to_markdown(&self, _: &mut ParseState) -> String {
        format!("${}$", self.body.0)
    }
}

impl MdxComponentResult for InlineMathResult {
    fn to_mdx_component(&self, res: &mut ParseState) -> String {
        self.to_jsx_component(res)
    }
}

impl ConundrumParser<InlineMathResult> for InlineMathResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<InlineMathResult> {
        let body = delimited('$', take_till(1.., |c: char| c == '$'), '$').parse_next(input)?;
        Ok(InlineMathResult { body: ConundrumString(body.to_string()) })
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

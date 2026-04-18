use serde::Serialize;
use typeshare::typeshare;
use winnow::{Parser, combinator::delimited, error::ErrMode, stream::AsChar, token::take_till};

use crate::{
    lang::{
        lib::ui::components::markdown::math::props::MathData,
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::ParseState,
            },
            traits::{
                conundrum_input::ConundrumInput, html_js_component_result::HtmlJsComponentResult,
                jsx_component_result::JsxComponentResult, markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult, plain_text_component_result::PlainTextComponentResult,
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
    fn to_plain_text(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(self.body.0.clone())
    }
}

impl JsxComponentResult for InlineMathResult {
    fn to_jsx_component(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        let math_data = MathData { display: false,
                                   content: self.body.0.clone(),
                                   id: None,
                                   idx: None };
        let s = serde_json::to_string(&math_data).map_err(|e| {
            eprintln!("Error: {:#?}", e);
            ErrMode::Backtrack(
                ConundrumErrorVariant::UserFacingGeneralParserError(
                    ConundrumError::from_msg_and_details("Serialization error", "Conundrum failed to serialize the math data provided to one of your inline equations.")
                )
            )
        })?;
        Ok(format!("<{} data={{{}}}/>", AutoInsertedComponentName::AutoInsertedMathBlock, s,))
    }
}

impl HtmlJsComponentResult for InlineMathResult {
    fn to_html_js_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        let math_string = self.body.to_math(false, res.trusted)?;
        Ok(format!("<span className=\"conundrum-math conundrum-math-inline\">{}</span>", math_string))
    }
}

impl MarkdownComponentResult for InlineMathResult {
    fn to_markdown(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(format!("${}$", self.body.0))
    }
}

impl MdxComponentResult for InlineMathResult {
    fn to_mdx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.to_jsx_component(res)
    }
}

impl ConundrumParser<InlineMathResult> for InlineMathResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<InlineMathResult> {
        let body = delimited('$', take_till(1.., |c: char| c == '$' || AsChar::is_newline(c)), '$').parse_next(input)?;
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

use serde::Serialize;
use winnow::{
    Parser,
    combinator::delimited,
    token::{literal, take_until},
};

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
    output::{
        general::component_constants::auto_inserted_component_name::AutoInsertedComponentName,
        output_components::output_utils::format_embedded_object_property,
    },
    parsers::{conundrum::logic::string::conundrum_string::ConundrumString, parser_trait::ConundrumParser},
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct BlockMathResult {
    pub body: ConundrumString,
}

impl PlainTextComponentResult for BlockMathResult {
    fn to_plain_text(&self, _: &mut ParseState) -> String {
        self.body.0.clone()
    }
}

impl ConundrumComponentResult for BlockMathResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> String {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else if res.is_markdown_or_search_or_ai() {
            self.to_markdown(res)
        } else if res.targets_jsx() {
            self.to_jsx_component(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MarkdownComponentResult for BlockMathResult {
    fn to_markdown(&self, res: &mut ParseState) -> String {
        format!("$$\n{}\n$$", self.body)
    }
}

impl JsxComponentResult for BlockMathResult {
    fn to_jsx_component(&self, res: &mut ParseState) -> String {
        let math_data = MathData { math: self.body.0.clone(),
                                   display: true };
        format!("<{} math={{{}}} />",
                AutoInsertedComponentName::AutoInsertedCodeBlock,
                serde_json::to_string(&math_data).unwrap_or_else(|_| String::from("{}")))
    }
}

impl HtmlJsComponentResult for BlockMathResult {
    fn to_html_js_component(&self, res: &mut ParseState) -> String {
        format!("<div className=\"conundrum-math conundrum-math-block\">\n$${}$$\n</div>", self.body)
    }
}

impl MdxComponentResult for BlockMathResult {
    fn to_mdx_component(&self, res: &mut ParseState) -> String {
        self.to_jsx_component(res)
    }
}

impl ConundrumParser<BlockMathResult> for BlockMathResult {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<BlockMathResult> {
        let body = delimited(literal("$$"), take_until(1.., "$$"), literal("$$")).parse_next(input)?;
        Ok(BlockMathResult { body: ConundrumString(body.to_string()) })
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
    fn parses_block_math_content() {
        let test_content = "$$\n\ne=mc^2\n\n$$";
        let mut test_props = wrap_test_conundrum_content(test_content);
        let res =
            BlockMathResult::parse_input_string(&mut test_props).expect("Parses math block without throwing an error.");
        assert_eq!(res.body, "\n\ne=mc^2\n\n", "Finds the proper math body when parsing inline math.");
    }
}

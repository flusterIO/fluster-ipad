use crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult;
use crate::lang::runtime::state::parse_state::ParseState;
use crate::lang::runtime::traits::conundrum_input::{ConundrumInput, get_conundrum_input};
use crate::lang::runtime::traits::fluster_component_result::ConundrumComponentResult;
use crate::lang::runtime::traits::jsx_component_result::JsxComponentResult;
use crate::parsers::javascript::javascript_parser_trait::JavascriptParser;
use serde::Serialize;
use winnow::Parser;
use winnow::combinator::{alt, delimited};
use winnow::token::take_till;

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct JavascriptStringResult {
    pub value: String,
    pub delimiter: char,
}

pub fn double_quoted_javascript_string(input: &mut ConundrumInput) -> ConundrumModalResult<JavascriptStringResult> {
    let res = delimited('"', take_till(0.., |c| c == '\n' || c == '"'), '"').parse_next(input)?;
    Ok(JavascriptStringResult { value: res.to_string(),
                                delimiter: '"' })
}

pub fn single_quoted_javascript_string(input: &mut ConundrumInput) -> ConundrumModalResult<JavascriptStringResult> {
    let res = delimited('\'', take_till(0.., |c| c == '\n' || c == '\''), '\'').parse_next(input)?;
    Ok(JavascriptStringResult { value: res.to_string(),
                                delimiter: '\'' })
}

pub fn back_tick_quoted_javascript_string(input: &mut ConundrumInput) -> ConundrumModalResult<JavascriptStringResult> {
    let res = delimited('`', take_till(0.., |c| c == '`'), '`').parse_next(input)?;
    Ok(JavascriptStringResult { value: res.to_string(),
                                delimiter: '`' })
}

pub fn single_or_double_quoted_string(input: &mut ConundrumInput) -> ConundrumModalResult<JavascriptStringResult> {
    alt((single_quoted_javascript_string, double_quoted_javascript_string)).parse_next(input)
}

impl JavascriptParser<JavascriptStringResult> for JavascriptStringResult {
    fn parse_javascript(input: &mut ConundrumInput) -> ConundrumModalResult<JavascriptStringResult> {
        let s = alt((back_tick_quoted_javascript_string,
                     single_quoted_javascript_string,
                     double_quoted_javascript_string)).parse_next(input)?;
        Ok(s)
    }
}

impl JsxComponentResult for JavascriptStringResult {
    fn to_jsx_component(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(format!("{}{}{}", self.delimiter, self.value, self.delimiter))
    }
}

impl ConundrumComponentResult for JavascriptStringResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.is_markdown_or_plain_text() {
            Ok(String::from(""))
        } else {
            self.to_jsx_component(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_multi_line_back_tick_quoted_javascript_string() {
        let test_content = r#"`My test ${here}
content here`"#;
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res = JavascriptStringResult::parse_javascript(&mut test_data).expect("Parses javascript string without throwing an error.");

        println!("Res: {}", res.value);

        assert!(
                res.value
                == r#"My test ${here}
content here"#,
                "Returns the proper value"
        );
    }

    #[test]
    fn parses_simple_back_tick_quoted_javascript_string() {
        let test_content = r#"`My test content here`"#;
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res = JavascriptStringResult::parse_javascript(&mut test_data).expect("Parses javascript string without throwing an error.");

        assert!(res.value == "My test content here", "Returns the proper value");
    }

    #[test]
    fn parses_simple_single_quoted_javascript_string() {
        let test_content = r#"'My test content here'"#;
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res = JavascriptStringResult::parse_javascript(&mut test_data).expect("Parses javascript string without throwing an error.");

        assert!(res.value == "My test content here", "Returns the proper value");
    }

    #[test]
    fn parses_simple_double_quoted_javascript_string() {
        let test_content = r#""My test content here""#;
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res = JavascriptStringResult::parse_javascript(&mut test_data).expect("Parses javascript string without throwing an error.");

        assert!(res.value == "My test content here", "Returns the proper value");
    }
}

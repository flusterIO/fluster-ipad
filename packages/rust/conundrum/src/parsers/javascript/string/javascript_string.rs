use crate::lang::runtime::traits::conundrum_input::ConundrumInput;
use crate::parsers::javascript::javascript_parser_trait::JavascriptParser;
use serde::Serialize;
use winnow::combinator::{alt, delimited};
use winnow::token::take_till;
use winnow::{ModalResult, Parser};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct JavascriptStringResult {
    pub value: String,
}

pub fn double_quoted_javascript_string(input: &mut ConundrumInput) -> ModalResult<JavascriptStringResult> {
    let res = delimited('"', take_till(0.., |c| c == '\n' || c == '"'), '"').parse_next(input)?;
    Ok(JavascriptStringResult { value: res.to_string() })
}

pub fn single_quoted_javascript_string(input: &mut ConundrumInput) -> ModalResult<JavascriptStringResult> {
    let res = delimited('\'', take_till(0.., |c| c == '\n' || c == '\''), '\'').parse_next(input)?;
    Ok(JavascriptStringResult { value: res.to_string() })
}

pub fn back_tick_quoted_javascript_string(input: &mut ConundrumInput) -> ModalResult<JavascriptStringResult> {
    let res = delimited('`', take_till(0.., |c| c == '`'), '`').parse_next(input)?;
    Ok(JavascriptStringResult { value: res.to_string() })
}

pub fn single_or_double_quoted_string(input: &mut ConundrumInput) -> ModalResult<JavascriptStringResult> {
    alt((single_quoted_javascript_string, double_quoted_javascript_string)).parse_next(input)
}

impl JavascriptParser<JavascriptStringResult> for JavascriptStringResult {
    fn parse_javascript(input: &mut ConundrumInput) -> ModalResult<JavascriptStringResult> {
        let s = alt((back_tick_quoted_javascript_string,
                     single_quoted_javascript_string,
                     double_quoted_javascript_string)).parse_next(input)?;
        Ok(s)
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

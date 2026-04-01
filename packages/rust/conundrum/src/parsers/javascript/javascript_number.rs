use serde::Serialize;
use winnow::ascii::{dec_int, float};
use winnow::{ModalResult, Parser, combinator::alt};

use crate::lang::runtime::traits::conundrum_input::ConundrumInput;
use crate::parsers::conundrum::logic::number::conundrum_number::ConundrumNumber;
use crate::parsers::javascript::javascript_parser_trait::JavascriptParser;

/// Creating the distinction between ints and floats will be important as the
/// runtime will execute mostly in Rust, and being able to parse some of the
/// mdx, javascript inputs will be super useful.
#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct JavascriptNumberResult {
    pub value: ConundrumNumber,
}

impl PartialEq<i128> for JavascriptNumberResult {
    fn eq(&self, other: &i128) -> bool {
        match self.value {
            ConundrumNumber::Float(_) => false,
            ConundrumNumber::Int(n) => n == *other,
        }
    }
}

impl PartialEq<f64> for JavascriptNumberResult {
    fn eq(&self, other: &f64) -> bool {
        let n = self.value.as_float();
        n == *other
    }
}

pub fn javascript_int(input: &mut ConundrumInput) -> ModalResult<JavascriptNumberResult> {
    let n: i128 = dec_int.parse_next(input)?;
    Ok(JavascriptNumberResult { value: ConundrumNumber::Int(n) })
}

pub fn javascript_float(input: &mut ConundrumInput) -> ModalResult<JavascriptNumberResult> {
    let x: f64 = float.parse_next(input)?;
    Ok(JavascriptNumberResult { value: ConundrumNumber::Float(x) })
}

impl JavascriptParser<JavascriptNumberResult> for JavascriptNumberResult {
    fn parse_javascript(input: &mut ConundrumInput) -> ModalResult<JavascriptNumberResult> {
        alt((javascript_float, javascript_int)).parse_next(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_javascript_int() {
        let test_input = "134134";
        let mut test_data = wrap_test_conundrum_content(test_input);
        let res = javascript_int.parse_next(&mut test_data);
        assert!(
                res.as_ref().is_ok_and(|x| *x == 134134),
                "Parses a javascript-like float and returns the proper
        value."
        );

        // assert_eq!(result, 4);
    }

    #[test]
    fn parses_javascript_float() {
        let test_input = "134134.34131";
        let mut test_data = wrap_test_conundrum_content(test_input);
        let res = javascript_float.parse_next(&mut test_data);
        assert!(res.as_ref().is_ok_and(|x| *x == 134134.34131),
                "Parses a javascript-like float and returns the proper value.");

        // assert_eq!(result, 4);
    }
}

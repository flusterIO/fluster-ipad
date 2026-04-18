use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            state::conundrum_error_variant::ConundrumModalResult,
            traits::conundrum_input::{ConundrumInput, get_conundrum_input},
        },
    },
    parsers::{
        conundrum::logic::{bool::boolean::ConundrumBoolean, token::ConundrumLogicToken},
        javascript::{
            javascript_parser_trait::JavascriptParser, object::javascript_key_value_pair::JavascriptObjectKeyValuePair,
        },
        react::parser_components::jsx_properties::{
            jsx_curly_bracket_wrapped_property::jsx_curly_bracket_wrapped_property, jsx_property::JsxPropertyParser,
            jsx_property_key::jsx_property_key,
        },
    },
};
use serde::Serialize;
use winnow::combinator::alt;
use winnow::{Parser, stream::Stream};

#[derive(Debug, Serialize)]
pub struct JsxBooleanPropertyResult {}

pub fn parse_full_boolean_property(input: &mut ConundrumInput) -> ConundrumModalResult<JavascriptObjectKeyValuePair> {
    let start = input.input.checkpoint();
    let (key, wrapped_content) = jsx_curly_bracket_wrapped_property.parse_next(input).inspect_err(|_| {
                                                                                          input.input.reset(&start);
                                                                                      })?;
    let state = input.state.borrow();
    let mut wrapped_content_input = get_conundrum_input(&wrapped_content, state.clone());
    let value =
        ConundrumBoolean::parse_javascript.map(ConundrumLogicToken::Bool).parse_next(&mut wrapped_content_input)?;

    Ok(JavascriptObjectKeyValuePair { key,
                                      value: Box::new(ParsedElement::Logic(value)) })
}

impl JsxPropertyParser for JsxBooleanPropertyResult {
    fn parse_jsx_property(input: &mut ConundrumInput) -> ConundrumModalResult<JavascriptObjectKeyValuePair> {
        alt((parse_full_boolean_property,
                       jsx_property_key.map(|key| JavascriptObjectKeyValuePair { key,
                                                                                 value: Box::new(
                                                                                     ParsedElement::Logic(
                                                                                         ConundrumLogicToken::Bool(ConundrumBoolean (true ))
                                                                                     )
                                                                                 )})
                                                                                 )).parse_next(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_simple_jsx_boolean_property() {
        let test_content = r#"myBool3"#;

        let mut test_data = wrap_test_conundrum_content(test_content);

        let res = JsxBooleanPropertyResult::parse_jsx_property(&mut test_data).expect("Parses valid boolean property without throwing an error.");

        assert!(match res.value.as_ref() {
                    ParsedElement::Logic(l) => match l {
                        ConundrumLogicToken::Bool(b) => b.0,
                        _ => false,
                    },
                    _ => false,
                },
                "Finds the proper value");

        assert!(res.key == "myBool3", "Finds the proper boolean key");
        assert!(test_data.input.is_empty(), "Consumes the entire input.");
    }

    #[test]
    fn parses_jsx_boolean_property_multi_line() {
        let test_content = r#"myBool2={ 
            true
}"#;

        let mut test_data = wrap_test_conundrum_content(test_content);

        let res = JsxBooleanPropertyResult::parse_jsx_property(&mut test_data).expect("Parses valid boolean property without throwing an error.");

        assert!(match res.value.as_ref() {
                    ParsedElement::Logic(l) => match l {
                        ConundrumLogicToken::Bool(b) => b.0,
                        _ => false,
                    },
                    _ => false,
                },
                "Finds the proper value");

        assert!(res.key == "myBool2", "Finds the proper boolean key");
        assert!(test_data.input.is_empty(), "Consumes the entire input.");
    }

    #[test]
    fn parses_jsx_boolean_property_with_no_space() {
        let test_content = "myBool={false}";

        let mut test_data = wrap_test_conundrum_content(test_content);

        let res = JsxBooleanPropertyResult::parse_jsx_property(&mut test_data).expect("Parses valid boolean property without throwing an error.");

        assert!(match res.value.as_ref() {
                    ParsedElement::Logic(l) => match l {
                        ConundrumLogicToken::Bool(b) => !b.0,
                        _ => false,
                    },
                    _ => false,
                },
                "Finds the proper value");

        assert!(res.key == "myBool", "Finds the proper boolean key");
        assert!(test_data.input.is_empty(), "Consumes the entire input.");
    }

    #[test]
    fn parses_jsx_boolean_property() {
        let test_content = "myBool={ false}";

        let mut test_data = wrap_test_conundrum_content(test_content);

        let res = JsxBooleanPropertyResult::parse_jsx_property(&mut test_data).expect("Parses valid boolean property without throwing an error.");

        assert!(match res.value.as_ref() {
                    ParsedElement::Logic(l) => match l {
                        ConundrumLogicToken::Bool(b) => !b.0,
                        _ => false,
                    },
                    _ => false,
                },
                "Finds the proper value");

        assert!(res.key == "myBool", "Finds the proper boolean key");
        assert!(test_data.input.is_empty(), "Consumes the entire input.");
    }
}

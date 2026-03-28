use crate::{
    lang::runtime::traits::conundrum_input::{ConundrumInput, get_conundrum_input},
    parsers::{
        javascript::{javascript_boolean::JavascriptBooleanResult, javascript_parser_trait::JavascriptParser},
        react::parser_components::jsx_properties::{
            jsx_curly_bracket_wrapped_property::jsx_curly_bracket_wrapped_property, jsx_property::JsxPropertyParser,
            jsx_property_key::jsx_property_key,
        },
    },
};
use serde::Serialize;
use winnow::combinator::alt;
use winnow::{ModalResult, Parser, stream::Stream};

#[derive(Debug, Serialize)]
pub struct JsxBooleanPropertyResult {
    pub key: String,
    pub value: bool,
}

pub fn parse_full_boolean_property(input: &mut ConundrumInput) -> ModalResult<JsxBooleanPropertyResult> {
    let start = input.input.checkpoint();
    let (key, wrapped_content) = jsx_curly_bracket_wrapped_property.parse_next(input).inspect_err(|_| {
                                                                                          input.input.reset(&start);
                                                                                      })?;

    println!("Wrapped content: '{}'", wrapped_content);
    let state = input.state.borrow();
    let mut wrapped_content_input = get_conundrum_input(&wrapped_content, state.modifiers.clone());
    let js_res = JavascriptBooleanResult::parse_javascript.parse_next(&mut wrapped_content_input)?;

    Ok(JsxBooleanPropertyResult { key,
                                  value: js_res.value })
}

impl JsxPropertyParser<JsxBooleanPropertyResult> for JsxBooleanPropertyResult {
    fn parse_jsx_property(input: &mut ConundrumInput) -> ModalResult<JsxBooleanPropertyResult> {
        let start = input.input.checkpoint();
        let res = alt((parse_full_boolean_property,
                       jsx_property_key.map(|s| JsxBooleanPropertyResult { key: s,
                                                                           value: true }))).parse_next(input)
                                                                                           .inspect_err(|_| {
                                                                                               input.input
                                                                                                    .reset(&start);
                                                                                           })?;

        Ok(res)
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

        println!("Response: {:#?}", res);

        assert!(res.value, "Finds the proper value");

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

        println!("Response: {:#?}", res);

        assert!(res.value, "Finds the proper value");

        assert!(res.key == "myBool2", "Finds the proper boolean key");
        assert!(test_data.input.is_empty(), "Consumes the entire input.");
    }

    #[test]
    fn parses_jsx_boolean_property_with_no_space() {
        let test_content = "myBool={false}";

        let mut test_data = wrap_test_conundrum_content(test_content);

        let res = JsxBooleanPropertyResult::parse_jsx_property(&mut test_data).expect("Parses valid boolean property without throwing an error.");

        println!("Response: {:#?}", res);

        assert!(!res.value, "Finds the proper value");

        assert!(res.key == "myBool", "Finds the proper boolean key");
        assert!(test_data.input.is_empty(), "Consumes the entire input.");
    }

    #[test]
    fn parses_jsx_boolean_property() {
        let test_content = "myBool={ false}";

        let mut test_data = wrap_test_conundrum_content(test_content);

        let res = JsxBooleanPropertyResult::parse_jsx_property(&mut test_data).expect("Parses valid boolean property without throwing an error.");

        println!("Response: {:#?}", res);

        assert!(!res.value, "Finds the proper value");

        assert!(res.key == "myBool", "Finds the proper boolean key");
        assert!(test_data.input.is_empty(), "Consumes the entire input.");
    }
}

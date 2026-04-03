use winnow::{ModalResult, Parser, stream::Stream};

use crate::{
    lang::runtime::traits::conundrum_input::{ConundrumInput, get_conundrum_input},
    parsers::{
        javascript::{
            javascript_parser_trait::JavascriptParser,
            object::{
                javascript_key_value_pair::JavascriptObjectKeyValuePair, javascript_object::JavascriptObjectResult,
            },
            parsed_javascript_elements::ParsedJavascriptElement,
        },
        react::parser_components::jsx_properties::{
            jsx_curly_bracket_wrapped_property::jsx_curly_bracket_wrapped_property, jsx_property::JsxPropertyParser,
        },
    },
};

pub struct JsxObjectPropertyResult {}

// impl JsxPropertyParser for JsxObjectPropertyResult {
//     fn parse_jsx_property(input: &mut ConundrumInput) ->
// ConundrumResult<JavascriptObjectKeyValuePair> {         let start =
// input.input.checkpoint();         let (key, wrapped_content) =
// jsx_curly_bracket_wrapped_property.parse_next(input).inspect_err(|_| {
//
// input.input.reset(&start);
// })?;

//         let state = input.state.borrow();

//         let mut new_input = get_conundrum_input(&wrapped_content,
// state.modifiers.clone());

//         let object_data = JavascriptObjectResult::parse_javascript(&mut
// new_input)?;

//         Ok(JavascriptObjectKeyValuePair { key,
//                                           value:
// Box::new(ParsedJavascriptElement::Object(object_data)) })     }
// }

// #[cfg(test)]
// mod tests {
//     use fluster_core_utilities::core_types::fluster_error::FlusterError;

//     use crate::testing::wrap_test_content::wrap_test_conundrum_content;

//     use super::*;

//     #[test]
//     fn parses_jsx_object_prop() {
//         let test_content = r#"myObject={{
//         myKey: "my value with a }",
//         myOtherKey: 3.14,
//         myThirdKey: false
//         }}"#;

//         let mut test_data = wrap_test_conundrum_content(test_content);

//         let res = JsxObjectPropertyResult::parse_jsx_property(&mut
// test_data).expect("Parses object without throwing an error.");

//         println!("Res: {:#?}", res);

//         assert!(res.key == "myObject", "Returns the proper object key");
//         let value = match *res.value {
//             ParsedJavascriptElement::Object(n) => Ok(n),
//             _ => Err(FlusterError::ConundrumParsingError),
//         };
//         assert!(value.is_ok(), "Returns an object type.");
//         let x = value.unwrap();
//         let string_value = x.data.get("myKey");

//         assert!(string_value.is_some_and(|s| match s {
//                                 ParsedJavascriptElement::String(j) => j.value
// == "my value with a }",                                 _ => false,
//                             }),
//                 "Finds the proper string value");

//         let num_value = x.data.get("myOtherKey");

//         assert!(num_value.is_some_and(|s| match s {
//                              ParsedJavascriptElement::Number(j) =>
// j.value.as_float() == 3.14,                              _ => false,
//                          }),
//                 "Finds the proper number value");

//         let bool_value = x.data.get("myThirdKey");

//         assert!(bool_value.is_some_and(|s| match s {
//                               ParsedJavascriptElement::Boolean(j) =>
// !j.value,                               _ => false,
//                           }),
//                 "Finds the proper number value");

//         // assert_eq!(result, 4);
//     }
// }

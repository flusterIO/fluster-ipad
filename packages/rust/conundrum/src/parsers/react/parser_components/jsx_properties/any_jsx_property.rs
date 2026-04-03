use winnow::combinator::alt;
use winnow::{ModalResult, Parser};

use crate::lang::runtime::state::conundrum_error_variant::ConundrumResult;
use crate::parsers::javascript::javascript_boolean::JavascriptBooleanResult;
use crate::parsers::javascript::object::javascript_key_value_pair::JavascriptObjectKeyValuePair;
use crate::parsers::javascript::parsed_javascript_elements::ParsedJavascriptElement;
use crate::parsers::react::parser_components::jsx_properties::jsx_curly_bracket_wrapped_property::any_curly_bracket_jsx_property;
use crate::parsers::react::parser_components::jsx_properties::jsx_property_key::jsx_property_key;
use crate::parsers::react::parser_components::jsx_properties::string_property::JsxStringPropertyResult;
use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::react::parser_components::jsx_properties::jsx_property::JsxPropertyParser,
};

pub fn any_jsx_property(input: &mut ConundrumInput) -> ConundrumResult<JavascriptObjectKeyValuePair> {
    alt((JsxStringPropertyResult::parse_jsx_property,
         any_curly_bracket_jsx_property,
         jsx_property_key.map(|key| {
                             JavascriptObjectKeyValuePair { key,
                                                                   value: Box::new(ParsedJavascriptElement::Boolean(
                                                                           JavascriptBooleanResult {
                                                                               value: true
                                                                           }
                                                                   )) }
                         }))).parse_next(input)
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use fluster_core_utilities::core_types::fluster_error::FlusterError;
    use winnow::Parser;

    use crate::{
        lang::elements::parsed_elements::ParsedElement,
        parsers::{
            javascript::parsed_javascript_elements::ParsedJavascriptElement,
            react::parser_components::jsx_properties::jsx_curly_bracket_wrapped_property::any_curly_bracket_jsx_property,
        },
        testing::wrap_test_content::wrap_test_conundrum_content,
    };

    #[test]
    fn parses_object_property() {
        let test_content = r#"myObject={{
        myKey: "my value with a }",
        myOtherKey: 3.14,
        myThirdKey: false
        }}"#;
        let mut test_data = wrap_test_conundrum_content(test_content);

        let res = any_curly_bracket_jsx_property.parse_next(&mut test_data)
                                                .expect("Parses object property without throwing an error.");
        println!("Res: {:#?}", res);

        assert!(res.key == "myObject", "Returns the proper object key");
        let value = match *res.value {
            ParsedJavascriptElement::Object(n) => Ok(n),
            _ => Err(FlusterError::ConundrumParsingError),
        };
        assert!(value.is_ok(), "Returns an object type.");
        let x = value.unwrap();
        let string_value = x.data.get("myKey");

        assert!(string_value.is_some_and(|s| match s.deref() {
                                ParsedElement::Javascript(js) => match js {
                                    ParsedJavascriptElement::String(j) => j.value == "my value with a }",
                                    _ => false,
                                },
                                _ => false,
                            }),
                "Finds the proper string value");

        let num_value = x.data.get("myOtherKey");

        assert!(num_value.is_some_and(|s| match s.deref() {
                             ParsedElement::Javascript(js) => match js {
                                 ParsedJavascriptElement::Number(j) => j.value.as_float() == 3.14,
                                 _ => false,
                             },
                             _ => false,
                         }),
                "Finds the proper number value");

        let bool_value = x.data.get("myThirdKey");

        assert!(bool_value.is_some_and(|s| match s.deref() {
                              ParsedElement::Javascript(js) => match js {
                                  ParsedJavascriptElement::Boolean(j) => !j.value,
                                  _ => false,
                              },
                              _ => false,
                          }),
                "Finds the proper number value");
    }
}

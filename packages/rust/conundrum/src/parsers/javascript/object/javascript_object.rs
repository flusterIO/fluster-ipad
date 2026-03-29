use std::collections::HashMap;

use serde::Serialize;
use winnow::{
    ModalResult, Parser,
    combinator::{delimited, separated},
};

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::{
        javascript::{
            javascript_parser_trait::JavascriptParser, object::javascript_key_value_pair::JavascriptObjectKeyValuePair,
            parsed_javascript_elements::ParsedJavascriptElement,
        },
        parser_components::white_space_delimited::white_space_delimited,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize)]
pub struct JavascriptObjectResult {
    pub data: HashMap<String, ParsedJavascriptElement>,
}

impl JavascriptObjectResult {
    pub fn from_kv_pair_vec(entries: Vec<JavascriptObjectKeyValuePair>) -> Self {
        let mut data = HashMap::new();
        for entry in entries {
            data.insert(entry.key, *entry.value);
        }
        JavascriptObjectResult { data }
    }
}

impl JavascriptParser<JavascriptObjectResult> for JavascriptObjectResult {
    fn parse_javascript(input: &mut ConundrumInput) -> ModalResult<JavascriptObjectResult> {
        let entries: Vec<JavascriptObjectKeyValuePair> =
            delimited('{',
                      separated(0.., white_space_delimited(JavascriptObjectKeyValuePair::parse_javascript), ','),
                      '}').parse_next(input)?;

        // PERFORMANCE: Move this to Dashmap once you can get on WIFI and install the
        // serde feature for Dashmap.
        let mut data: HashMap<String, ParsedJavascriptElement> = HashMap::new();

        for entry in entries {
            data.insert(entry.key, *entry.value);
        }

        Ok(JavascriptObjectResult { data })
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::{
        test_util_functions::hashmap_key_matches::hashmap_key_matches, wrap_test_content::wrap_test_conundrum_content,
    };

    use super::*;

    #[test]
    fn parses_javascript_object() {
        let test_content = r#"{
        myKey: "my value",
        myOtherKey: 3.14,
        myThirdKey: false,
        myObjectKey: {
            myNestedObject: "my nested value"
        }
        }"#;

        let mut test_data = wrap_test_conundrum_content(test_content);

        let res =
            JavascriptObjectResult::parse_javascript(&mut test_data).expect("Parses object without throwing an error.");

        hashmap_key_matches(&res.data, &"myKey".to_string(), "myKey", |v| match v {
            ParsedJavascriptElement::String(s) => s.value == "my value",
            _ => false,
        });

        hashmap_key_matches(&res.data, &"myOtherKey".to_string(), "myOtherKey", |v| match v {
            #[allow(clippy::approx_constant)]
            ParsedJavascriptElement::Number(n) => n.value.as_float() == 3.14,
            _ => false,
        });

        hashmap_key_matches(&res.data, &"myThirdKey".to_string(), "myThirdKey", |v| match v {
            ParsedJavascriptElement::Boolean(n) => !n.value,
            _ => false,
        });

        hashmap_key_matches(&res.data, &"myObjectKey".to_string(), "myObjectKey", |v| match v {
            ParsedJavascriptElement::Object(o) => true, // Not going to
            // bother testing
            // any deeper
            // yet, but that
            // definitely
            // needs to be
            // taken care of.
            _ => false,
        });
    }
}

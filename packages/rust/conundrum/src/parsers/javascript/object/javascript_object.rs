use dashmap::DashMap;
use serde::Serialize;
use winnow::{
    Parser,
    combinator::{delimited, separated},
};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            state::{conundrum_error_variant::ConundrumResult, parse_state::ParseState},
            traits::{
                conundrum_input::ConundrumInput, fluster_component_result::ConundrumComponentResult,
                jsx_component_result::JsxComponentResult, mdx_component_result::MdxComponentResult,
            },
        },
    },
    parsers::{
        conundrum::logic::string::conundrum_string::ConundrumString,
        javascript::{
            javascript_parser_trait::JavascriptParser, object::javascript_key_value_pair::JavascriptObjectKeyValuePair,
        },
        parser_components::white_space_delimited::white_space_delimited,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct JavascriptObjectResult {
    pub data: DashMap<String, ParsedElement>,
}

impl JavascriptObjectResult {
    pub fn from_kv_pair_vec(entries: Vec<JavascriptObjectKeyValuePair>) -> Self {
        let data = DashMap::new();
        for entry in entries {
            data.insert(entry.key, *entry.value);
        }
        JavascriptObjectResult { data }
    }
}

impl JavascriptParser<JavascriptObjectResult> for JavascriptObjectResult {
    fn parse_javascript(input: &mut ConundrumInput) -> ConundrumResult<JavascriptObjectResult> {
        let entries: Vec<JavascriptObjectKeyValuePair> =
            delimited('{',
                      separated(0.., white_space_delimited(JavascriptObjectKeyValuePair::parse_javascript), ','),
                      '}').parse_next(input)?;

        let data: DashMap<String, ParsedElement> = DashMap::new();

        for entry in entries {
            data.insert(entry.key, *entry.value);
        }

        Ok(JavascriptObjectResult { data })
    }
}

impl JsxComponentResult for JavascriptObjectResult {
    fn to_jsx_component(&self, res: &mut ParseState) -> String {
        let mut s = String::from("{\n");
        for (k, v) in self.data.clone() {
            if let Ok(key) = ConundrumString::new(k.as_str()).to_quotable_string() {
                s += format!("\"{}\": {}", key, v.to_mdx_component(res)).as_str();
            }
        }
        s += "\n};";
        s.to_string()
    }
}

impl ConundrumComponentResult for JavascriptObjectResult {
    fn to_conundrum_component(&self, res: &mut crate::lang::runtime::state::parse_state::ParseState) -> String {
        if res.is_markdown_or_plain_text() {
            String::from("")
        } else {
            self.to_jsx_component(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parsers::javascript::parsed_javascript_elements::ParsedJavascriptElement,
        testing::{
            test_util_functions::hashmap_key_matches::hashmap_key_matches,
            wrap_test_content::wrap_test_conundrum_content,
        },
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
            ParsedElement::Javascript(js) => match js {
                ParsedJavascriptElement::String(s) => s.value == "my value",
                _ => false,
            },
            _ => false,
        });

        hashmap_key_matches(&res.data, &"myOtherKey".to_string(), "myOtherKey", |v| match v {
            #[allow(clippy::approx_constant)]
            ParsedElement::Javascript(js) => match js {
                ParsedJavascriptElement::Number(n) => n.value.as_float() == 3.14,
                _ => false,
            },
            _ => false,
        });

        hashmap_key_matches(&res.data, &"myThirdKey".to_string(), "myThirdKey", |v| match v {
            ParsedElement::Javascript(js) => match js {
                ParsedJavascriptElement::Boolean(n) => !n.value,
                _ => false,
            },
            _ => false,
        });

        hashmap_key_matches(&res.data, &"myObjectKey".to_string(), "myObjectKey", |v| match v {
            ParsedElement::Javascript(js) => match js {
                ParsedJavascriptElement::Object(n) => true,
                _ => false,
            },
            _ => false,
        });
    }
}

use serde::Serialize;

use crate::{
    lang::runtime::{
        state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState},
        traits::{conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult},
    },
    parsers::javascript::{
        function::javascript_function::JavascriptFunction,
        javascript_boolean::JavascriptBooleanResult,
        javascript_number::JavascriptNumberResult,
        object::{javascript_key_value_pair::JavascriptObjectKeyValuePair, javascript_object::JavascriptObjectResult},
        string::javascript_string::JavascriptStringResult,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum ParsedJavascriptElement {
    Boolean(JavascriptBooleanResult),
    Number(JavascriptNumberResult),
    String(JavascriptStringResult),
    Object(JavascriptObjectResult),
    KeyValuePair(JavascriptObjectKeyValuePair),
    Function(JavascriptFunction),
}

impl ConundrumComponentResult for ParsedJavascriptElement {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        match self {
            ParsedJavascriptElement::KeyValuePair(k) => k.to_conundrum_component(res),
            ParsedJavascriptElement::Number(n) => n.to_conundrum_component(res),
            ParsedJavascriptElement::Function(f) => f.to_conundrum_component(res),
            ParsedJavascriptElement::String(s) => s.to_conundrum_component(res),
            ParsedJavascriptElement::Boolean(b) => b.to_conundrum_component(res),
            ParsedJavascriptElement::Object(o) => o.to_conundrum_component(res),
        }
    }
}

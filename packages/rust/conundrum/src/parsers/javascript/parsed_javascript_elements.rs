use serde::Serialize;

use crate::parsers::javascript::{
    function::javascript_function::JavascriptFunction,
    javascript_boolean::JavascriptBooleanResult,
    javascript_number::JavascriptNumberResult,
    object::{javascript_key_value_pair::JavascriptObjectKeyValuePair, javascript_object::JavascriptObjectResult},
    string::javascript_string::JavascriptStringResult,
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

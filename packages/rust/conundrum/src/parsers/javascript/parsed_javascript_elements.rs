use serde::Serialize;

use crate::parsers::javascript::{
    javascript_boolean::JavascriptBooleanResult,
    javascript_number::JavascriptNumberResult,
    object::{javascript_key_value_pair::JavascriptObjectKeyValuePair, javascript_object::JavascriptObjectResult},
    string::javascript_string::JavascriptStringResult,
};

#[derive(Debug, Serialize)]
pub enum ParsedJavascriptElement {
    Boolean(JavascriptBooleanResult),
    Number(JavascriptNumberResult),
    String(JavascriptStringResult),
    Object(JavascriptObjectResult),
    KeyValuePair(JavascriptObjectKeyValuePair),
}

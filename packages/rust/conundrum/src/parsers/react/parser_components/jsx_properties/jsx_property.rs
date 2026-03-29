use winnow::ModalResult;

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::{
        javascript::object::javascript_key_value_pair::JavascriptObjectKeyValuePair,
        react::parser_components::jsx_properties::{
            boolean_property::JsxBooleanPropertyResult, string_property::JsxStringPropertyResult,
        },
    },
};

pub enum JsxProperty {
    Boolean(JsxBooleanPropertyResult),
    String(JsxStringPropertyResult),
    Text(String),
}

pub trait JsxPropertyParser {
    fn parse_jsx_property(input: &mut ConundrumInput) -> ModalResult<JavascriptObjectKeyValuePair>;
}

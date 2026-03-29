use winnow::combinator::alt;
use winnow::{ModalResult, Parser};

use crate::parsers::javascript::object::javascript_key_value_pair::JavascriptObjectKeyValuePair;
use crate::parsers::react::parser_components::jsx_properties::number_property::JsxNumberPropertyResult;
use crate::parsers::react::parser_components::jsx_properties::object_property::JsxObjectPropertyResult;
use crate::parsers::react::parser_components::jsx_properties::string_property::JsxStringPropertyResult;
use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::react::parser_components::jsx_properties::{
        boolean_property::JsxBooleanPropertyResult, jsx_property::JsxPropertyParser,
    },
};

pub fn any_jsx_property(input: &mut ConundrumInput) -> ModalResult<JavascriptObjectKeyValuePair> {
    alt((JsxStringPropertyResult::parse_jsx_property,
         JsxObjectPropertyResult::parse_jsx_property,
         JsxNumberPropertyResult::parse_jsx_property,
         JsxBooleanPropertyResult::parse_jsx_property /* Boolean must be last */)).parse_next(input)
}

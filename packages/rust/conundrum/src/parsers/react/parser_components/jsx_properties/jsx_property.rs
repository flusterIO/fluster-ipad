use winnow::ModalResult;

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::react::parser_components::jsx_properties::{
        boolean_property::JsxBooleanPropertyResult, string_property::JsxStringPropertyResult,
    },
};

pub enum JsxProperty {
    Boolean(JsxBooleanPropertyResult),
    String(JsxStringPropertyResult),
    Text(String),
}

pub trait JsxPropertyParser<T> {
    fn parse_jsx_property(input: &mut ConundrumInput) -> ModalResult<T>;
}

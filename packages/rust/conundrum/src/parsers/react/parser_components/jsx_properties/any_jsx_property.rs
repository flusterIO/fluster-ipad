use winnow::combinator::alt;
use winnow::token::any;
use winnow::{ModalResult, Parser};

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::react::parser_components::jsx_properties::{
        boolean_property::JsxBooleanPropertyResult,
        jsx_property::{JsxProperty, JsxPropertyParser},
    },
};

pub fn any_jsx_property(input: &mut ConundrumInput) -> ModalResult<JsxProperty> {
    let res = alt((JsxBooleanPropertyResult::parse_jsx_property.map(JsxProperty::Boolean),
                   any.map(|c: char| JsxProperty::Text(c.to_string())))).parse_next(input)?;

    Ok(res)
}

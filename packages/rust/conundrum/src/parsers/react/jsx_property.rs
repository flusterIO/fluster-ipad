use winnow::ModalResult;
use winnow::combinator::alt;

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::javascript::object::javascript_key_value_pair::JavascriptObjectKeyValuePair,
};

pub fn jsx_property(input: &mut ConundrumInput) -> ModalResult<JavascriptObjectKeyValuePair> {
    alt
}


use std::fmt::Display;

use winnow::{Parser, ascii::dec_int, error::{ContextError, ErrMode}};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement, lib::ui::ui_traits::jsx_prop_representable::{FromJsxPropsOptional, JsxPropRepresentable}, runtime::{state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        }, traits::conundrum_input::ConundrumInput}
    },
    parsers::{conundrum::logic::{number::conundrum_number::ConundrumNumber, object::object::ConundrumObject, token::ConundrumLogicToken}, parser_trait::ConundrumParser},
};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, Hash)]
pub struct ConundrumInt(pub i64);

uniffi::custom_newtype!(ConundrumInt, i64);


impl Eq for ConundrumInt {

}

impl PartialEq<ConundrumInt> for ConundrumInt {
    fn eq(&self, other: &ConundrumInt) -> bool {
        self.0 == (*other).0
    }
}
impl PartialEq<i64> for ConundrumInt {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl Display for ConundrumInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
} 

impl ConundrumParser<ConundrumInt> for ConundrumInt {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumInt> {
        let n: i64 = dec_int.parse_next(input).map_err(|_: ContextError| {
            ErrMode::Backtrack(
                ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid number", "Conundrum was looking for an integer and found something else."))
            )
        })?;
        Ok(ConundrumInt(n))
    }

    fn matches_first_char(_: char) -> bool {
        true
    }
}

impl FromJsxPropsOptional for ConundrumInt {
    fn from_jsx_props(props: &ConundrumObject, key: &str) -> ConundrumModalResult<Self>
        where Self: Sized {
        props.data.get(key).map(|n| {
            match n.value() {
                ParsedElement::Logic(l) => match l { 
                    ConundrumLogicToken::Number(n) => match n {
                        ConundrumNumber::Int(l) => Some(l.clone()),
                        _ => None
                    },
                    _ => None
                },
                _ => None
            }
            }).flatten().ok_or_else(|| {
                    ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Mismatched property", format!("Conundrum was looking for an integer at the {} key but found something else.", key).as_str())))
            })
    }
}

impl JsxPropRepresentable for ConundrumInt {
    fn to_jsx_prop(&self, key: &str) -> String {
        format!("{}={{{}}}", key, self.0)
    }
}

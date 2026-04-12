use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement, lib::ui::ui_traits::jsx_prop_representable::{FromJsxPropsOptional, JsxPropRepresentable}, runtime::state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        }
    },
    parsers::conundrum::logic::{number::conundrum_number::ConundrumNumber, object::object::ConundrumObject, token::ConundrumLogicToken},
};

#[derive(Debug, serde::Serialize, Clone, Copy)]
pub struct ConundrumInt(pub i64);

uniffi::custom_newtype!(ConundrumInt, i64);

impl PartialEq<i64> for ConundrumInt {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
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

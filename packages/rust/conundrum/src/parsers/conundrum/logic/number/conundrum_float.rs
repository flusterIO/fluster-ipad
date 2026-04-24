use uniffi::TypeId;
use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        runtime::state::{conundrum_error::ConundrumError, conundrum_error_variant::ConundrumErrorVariant},
    },
    parsers::conundrum::logic::{number::conundrum_number::ConundrumNumber, token::ConundrumLogicToken},
};
#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone, Copy)]
pub struct ConundrumFloat(pub f64);

impl FromJsxPropsOptional for ConundrumFloat {
    fn from_jsx_props(props: &crate::parsers::conundrum::logic::object::object::ConundrumObject,
                      key: &str)
                      -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<Self>
        where Self: Sized {
        if let Some(res) = props.data.get(key) {
            match res.value() {
                    ParsedElement::Logic(l) => match l {
                        ConundrumLogicToken::Number(n) => match n {
                            ConundrumNumber::Float(f) => Some(f),
                            _ => None
                        },
                        _ => None
                    },
                    _ => None
                }.cloned().ok_or_else(|| {
                    ErrMode::Backtrack(
                        ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Invalid float"))
                    )
                })
        } else {
            Err(ErrMode::Backtrack(ConundrumErrorVariant::KeyNotFound))
        }
    }
}

uniffi::custom_newtype!(ConundrumFloat, f64);

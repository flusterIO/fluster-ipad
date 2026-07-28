use num_traits::ToPrimitive;
use serde::Deserialize;
use surrealdb_types::SurrealValue;
use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        runtime::state::{conundrum_error::ConundrumError, conundrum_error_variant::ConundrumErrorVariant},
    },
    parsers::conundrum::logic::{number::conundrum_number::ConundrumNumber, token::ConundrumLogicToken},
};

/// ## TODO:
/// - [ ] Move this to dashu asap. Everything down to the database package is
///   depeneding on that
/// sweet arbitrary precision.
#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Deserialize, Clone, Copy)]
pub struct ConundrumFloat(pub f64);

impl SurrealValue for ConundrumFloat {
    fn kind_of() -> surrealdb_types::Kind {
        surrealdb_types::Kind::Decimal
    }

    fn into_value(self) -> surrealdb_types::Value {
        surrealdb_types::Value::from_f64(self.0)
    }

    fn from_value(value: surrealdb_types::Value) -> Result<Self, surrealdb::Error>
        where Self: Sized {
        if let Some(n) = value.as_decimal() {
            if let Some(unwrapped) = n.to_f64() {
                Ok(Self(unwrapped))
            } else {
                Err(surrealdb::Error::thrown("Ivalid ConundrumFloat encountered in the database.".to_string()))
            }
        } else {
            Err(surrealdb::Error::thrown("Ivalid ConundrumFloat encountered in the database.".to_string()))
        }
    }
}

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

impl PartialEq<f64> for ConundrumFloat {
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other
    }
}

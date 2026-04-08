use std::{ops::Deref, str::FromStr};

use serde::Serialize;
use strum::{EnumIter, IntoEnumIterator};
use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::ui_traits::jsx_prop_representable::{FromJsxPropsOptional, JsxPropRepresentable},
        runtime::state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        },
    },
    parsers::{
        conundrum::logic::{object::object::ConundrumObject, token::ConundrumLogicToken},
        javascript::parsed_javascript_elements::ParsedJavascriptElement,
    },
};

/// A set of common styles that can be applied as a group to indicate common
/// scenarios. These will become customizable soon so you won't be limited to
/// the default color values.
/// The emphasis documentation can be viewed via the `Emphasis??` command.
#[typeshare::typeshare]
#[derive(Serialize, uniffi::Enum, EnumIter, PartialEq, Eq, Default, Debug, strum_macros::Display, Clone)]
pub enum Emphasis {
    #[serde(rename = "info")]
    #[strum(to_string = "info")]
    Info,
    #[serde(rename = "error")]
    #[strum(to_string = "error")]
    Error,
    #[serde(rename = "warn")]
    #[strum(to_string = "warn")]
    Warn,
    #[serde(rename = "success")]
    #[strum(to_string = "success")]
    Success,
    #[serde(rename = "primary")]
    #[strum(to_string = "primary")]
    #[default]
    Primary,
    #[serde(rename = "important")]
    #[strum(to_string = "important")]
    Important,
    #[serde(rename = "research")]
    #[strum(to_string = "research")]
    Research,
    #[serde(rename = "highlight")]
    #[strum(to_string = "highlight")]
    Highlight,
    #[serde(rename = "card")]
    #[strum(to_string = "card")]
    Card,
}

impl FromStr for Emphasis {
    type Err = ConundrumErrorVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for item in Emphasis::iter() {
            if item.to_string() == s {
                return Ok(item);
            }
        }
        Err(ConundrumErrorVariant::UserFacingMissingOrIncorrectProperty(ConundrumError::from_message(format!("An incorrect property was provided to 'emphasis'. We found \"{}\".",
                                                                                        s).as_str())))
    }
}

impl FromJsxPropsOptional for Emphasis {
    /// The `key` property doesn't matter at all for the Emphasis component as
    /// it can be applied as a series of booleans.
    fn from_jsx_props(data: &ConundrumObject, _: &str) -> ConundrumModalResult<Self>
        where Self: Sized {
        for emph in Emphasis::iter() {
            #[allow(clippy::collapsible_if)]
            if let Some(emphasis) = data.data.get(emph.to_string().as_str()) {
                return match emphasis.deref() {
                    ParsedElement::Javascript(js) => match js {
                        ParsedJavascriptElement::Boolean(b) => {
                            if b.value {
                                Ok(emph)
                            } else {
                                Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Failed to gather emphasis."))))
                            }
                        }
                        _ => {
                            Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Failed to gather emphasis."))))
                        }
                    },
                    ParsedElement::Logic(l) => match l {
                        ConundrumLogicToken::Bool(b) => {
                            if b.0 {
                                Ok(emph)
                            } else {
                                Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Failed to gather emphasis."))))
                            }
                        }
                        _ => {
                            Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Failed to gather emphasis."))))
                        }
                    },
                    _ => {
                        Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Failed to gather emphasis."))))
                    }
                };
            }
        }
        Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Failed to gather emphasis."))))
    }
}

impl JsxPropRepresentable for Emphasis {
    fn to_jsx_prop(&self, key: &str) -> String {
        format!("{}=\"{}\"", key, self)
    }
}

#[cfg(test)]
mod tests {
    use dashmap::DashMap;

    use crate::{
        lang::elements::parsed_elements::ParsedElement,
        parsers::javascript::javascript_boolean::JavascriptBooleanResult,
    };

    use super::*;

    #[test]
    fn gets_jsx_from_props() {
        Emphasis::iter().for_each(|emph| {
                            let data = DashMap::new();
                            data.insert(emph.to_string(),
                                        ParsedElement::Javascript(ParsedJavascriptElement::Boolean(JavascriptBooleanResult { value: true })));

                            let object = ConundrumObject {
                                data
                            };

                            let res =
                                Emphasis::from_jsx_props(&object, emph.to_string().as_str()).expect("Finds the emphasis when the emphasis exists.");

                            assert!(res == emph, "Returns the inserted emphasis.");
                        });
    }
}

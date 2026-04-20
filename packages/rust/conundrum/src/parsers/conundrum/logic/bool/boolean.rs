use std::fmt::Display;

use serde::Serialize;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::ui_traits::jsx_prop_representable::{FromJsxPropsOptional, JsxPropRepresentable},
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            },
            traits::{
                conundrum_input::{ArcState, ConundrumInput},
                fluster_component_result::ConundrumComponentResult,
            },
        },
    },
    parsers::{
        conundrum::logic::{object::object::ConundrumObject, token::ConundrumLogicToken},
        javascript::javascript_parser_trait::JavascriptParser,
    },
};
use winnow::{Parser, combinator::alt};
use winnow::{error::ErrMode, token::literal};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone, Copy)]
pub struct ConundrumBoolean(pub bool);

uniffi::custom_newtype!(ConundrumBoolean, bool);

impl Default for ConundrumBoolean {
    fn default() -> Self {
        Self(false)
    }
}

impl JavascriptParser<ConundrumBoolean> for ConundrumBoolean {
    fn parse_javascript(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumBoolean> {
        let res = alt((literal("true"), literal("false"))).parse_next(input)?;
        Ok(ConundrumBoolean(res == "true"))
    }
}

impl FromJsxPropsOptional for ConundrumBoolean {
    fn from_jsx_props(props: &ConundrumObject, key: &str) -> ConundrumModalResult<Self>
        where Self: Sized {
        let res = props.data.get(key).ok_or_else(|| ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message(format!("Conundrum failed to find a boolean value for the {} key.", key).as_str()))))?;
        match res.value() {
            ParsedElement::Logic(em) => match em {
                ConundrumLogicToken::Bool(b) => Some(b),
                _ => None
            },
            _ => None
        }.map(|c| *c).ok_or_else(|| ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message(format!("Conundrum failed to find a boolean value for the {} key.", key).as_str()))))
    }
}

impl JsxPropRepresentable for ConundrumBoolean {
    fn to_jsx_prop(&self, key: &str) -> String {
        format!("{}={{{}}}", key, match self.0 {
            true => "true",
            false => "false",
        })
    }
}

impl Display for ConundrumBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 {
            write!(f, "true")
        } else {
            write!(f, "false")
        }
    }
}

impl ConundrumComponentResult for ConundrumBoolean {
    fn to_conundrum_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        if self.0 {
            Ok(String::from("true"))
        } else {
            Ok(String::from("false"))
        }
    }
}

use std::fmt::Display;

use serde::Serialize;
use winnow::{
    Parser,
    ascii::{dec_int, float},
    combinator::alt,
    error::ErrMode,
};

use crate::{
    lang::{
        lib::ui::ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        runtime::{
            mem::mem::MemoryArc,
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
        conundrum::{
            conundrum_logic_parser::ConundrumLogicParser,
            logic::number::{conundrum_float::ConundrumFloat, conundrum_int::ConundrumInt},
        },
        javascript::javascript_parser_trait::JavascriptParser,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(tag = "tag", content = "content")]
pub enum ConundrumNumber {
    Int(ConundrumInt),
    Float(ConundrumFloat),
}

pub fn javascript_int(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumNumber> {
    let n: i64 = dec_int.parse_next(input)?;
    Ok(ConundrumNumber::Int(ConundrumInt(n)))
}

pub fn javascript_float(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumNumber> {
    let x: f64 = float.parse_next(input)?;
    Ok(ConundrumNumber::Float(ConundrumFloat(x)))
}

impl ConundrumLogicParser for ConundrumNumber {
    fn parse_conundrum(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumNumber> {
        alt((javascript_int, javascript_float)).parse_next(input)
    }
}
impl FromJsxPropsOptional for ConundrumNumber {
    fn from_jsx_props(props: &crate::parsers::conundrum::logic::object::object::ConundrumObject,
                      key: &str)
                      -> ConundrumModalResult<Self>
        where Self: Sized {
        if let Ok(res) = ConundrumInt::from_jsx_props(&props, &key) {
            Ok(ConundrumNumber::Int(res))
        } else if let Ok(from_float) = ConundrumFloat::from_jsx_props(&props, &key) {
            Ok(ConundrumNumber::Float(from_float))
        } else {
            Err(
                ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid property", format!("The `{}` key expects a number but found something else.", key).as_str())))
            )
        }
    }
}

impl Display for ConundrumNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ConundrumNumber::Float(f) => f.0.to_string(),
            ConundrumNumber::Int(n) => n.0.to_string(),
        })
    }
}

impl ConundrumNumber {
    /// Return the number as an f64, unchanged if already
    /// `ConundrumNumber::Float`, else just a simple type cast.
    pub fn as_float(&self) -> f64 {
        match self {
            ConundrumNumber::Int(n) => n.0 as f64,
            ConundrumNumber::Float(n) => n.0,
        }
    }
}

impl ConundrumComponentResult for ConundrumNumber {
    fn to_conundrum_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(self.to_string())
    }
}

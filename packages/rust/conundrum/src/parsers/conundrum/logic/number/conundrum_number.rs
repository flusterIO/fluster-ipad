use std::fmt::Display;

use serde::Serialize;

use crate::{
    lang::runtime::{
        state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState},
        traits::fluster_component_result::ConundrumComponentResult,
    },
    parsers::conundrum::logic::number::{conundrum_float::ConundrumFloat, conundrum_int::ConundrumInt},
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(tag = "tag", content = "content")]
pub enum ConundrumNumber {
    Int(ConundrumInt),
    Float(ConundrumFloat),
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
    fn to_conundrum_component(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(self.to_string())
    }
}

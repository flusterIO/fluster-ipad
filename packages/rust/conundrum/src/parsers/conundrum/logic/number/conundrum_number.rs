use std::fmt::Display;

use serde::Serialize;

use crate::lang::runtime::{
    state::parse_state::ParseState, traits::fluster_component_result::ConundrumComponentResult,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(tag = "tag", content = "content")]
pub enum ConundrumNumber {
    Int(i128),
    Float(f64),
}

impl ConundrumNumber {
    /// Return the number as an f64, unchanged if already
    /// `ConundrumNumber::Float`, else just a simple type cast.
    pub fn as_float(&self) -> f64 {
        match self {
            ConundrumNumber::Int(n) => *n as f64,
            ConundrumNumber::Float(n) => *n,
        }
    }
}

impl Display for ConundrumNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl ConundrumComponentResult for ConundrumNumber {
    fn to_conundrum_component(&self, _: &mut ParseState) -> String {
        self.to_string()
    }
}

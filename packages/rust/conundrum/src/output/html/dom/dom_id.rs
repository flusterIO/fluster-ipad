use std::{fmt::Display, str::FromStr};

use askama::FastWritable;
use serde::{Deserialize, Serialize};
use winnow::error::ErrMode;

use crate::lang::runtime::state::conundrum_error_variant::ConundrumErrorVariant;

#[typeshare::typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DOMId(String);

uniffi::custom_newtype!(DOMId, String);

impl FromStr for DOMId {
    type Err = ErrMode<ConundrumErrorVariant>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DOMId(s.to_string()))
    }
}

impl DOMId {
    pub fn new(id: String) -> Self {
        DOMId(id)
    }

    pub fn value(&self) -> String {
        self.0.clone()
    }

    pub fn to_heading_id(&self) -> String {
        format!("h-{}", self.0)
    }
}

impl Display for DOMId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FastWritable for DOMId {
    fn write_into<W: core::fmt::Write + ?Sized>(&self,
                                                dest: &mut W,
                                                values: &dyn askama::Values)
                                                -> askama::Result<()> {
        self.0.as_str().write_into(dest, values)
    }
}

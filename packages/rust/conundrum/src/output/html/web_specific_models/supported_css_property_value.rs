use std::fmt::{Display, Pointer};

use serde_with::SerializeDisplay;

use crate::parsers::conundrum::color::conundrum_color::ConundrumColor;

#[derive(SerializeDisplay)]
pub enum CSSPropertyValue {
    Color(ConundrumColor),
}

impl Display for CSSPropertyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Color(c) => c.fmt(f),
        }
    }
}

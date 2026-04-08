use std::fmt::Display;

use serde::Serialize;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumResult},
        },
    },
    parsers::{
        conundrum::logic::{
            number::{conundrum_int::ConundrumInt, conundrum_number::ConundrumNumber},
            object::object::ConundrumObject,
        },
        javascript::parsed_javascript_elements::ParsedJavascriptElement,
    },
};

/// A simple utility around the heading depth, 1-6 just to add some convenience
/// methods.
#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone, Copy)]
pub struct HeadingDepth(pub ConundrumInt);

impl HeadingDepth {
    pub fn from_props(props: &ConundrumObject, key: &str) -> ConundrumResult<Self> {
        if let Some(res) = props.data.get(key) {
            let user_number = match res.value() {
                ParsedElement::Javascript(js) => match js {
                    ParsedJavascriptElement::Number(n) => match n.value {
                        ConundrumNumber::Int(n) => Some(n),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            };
            if let Some(un) = user_number {
                Ok(HeadingDepth(un))
            } else {
                Err(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Property provided for markdown heading depth does not look like a number.")))
            }
        } else {
            Err(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("No markdown heading depth provided.")))
        }
    }
}

impl Default for HeadingDepth {
    fn default() -> Self {
        Self(ConundrumInt(3))
    }
}

impl Display for HeadingDepth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("");
        for _ in 0..self.0.0 {
            s += "#"
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_depth() {
        let input = HeadingDepth(ConundrumInt(4));
        let res = input.to_string();
        assert!(res == "####", "Formats headings of the proper length");
    }
}

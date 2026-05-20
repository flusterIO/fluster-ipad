use std::{fmt::Display, sync::Arc};

use parking_lot::RwLock;
use serde::Serialize;
use winnow::{
    Parser,
    ascii::{dec_int, float},
    error::ErrMode,
    stream::{AsChar, Stream},
    token::take_till,
};

use crate::{
    lang::{
        lib::ui::ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::ParseState,
            },
            traits::{
                conundrum_input::{ArcState, ConundrumInput},
                fluster_component_result::ConundrumComponentResult,
            },
        },
    },
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
    parsers::conundrum::{
        conundrum_logic_parser::ConundrumLogicParser,
        logic::number::{conundrum_float::ConundrumFloat, conundrum_int::ConundrumInt},
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone, Copy)]
#[serde(tag = "tag", content = "content")]
pub enum ConundrumNumber {
    Int(ConundrumInt),
    Float(ConundrumFloat),
}

pub fn conundrum_int(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumNumber> {
    let n: i64 = dec_int.parse_next(input)?;
    Ok(ConundrumNumber::Int(ConundrumInt(n)))
}

pub fn conundrum_float(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumNumber> {
    let x: f64 = float.parse_next(input)?;
    Ok(ConundrumNumber::Float(ConundrumFloat(x)))
}

impl ConundrumLogicParser for ConundrumNumber {
    fn parse_conundrum(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumNumber> {
        let start = input.input.checkpoint();
        let res = take_till(1.., |c| !AsChar::is_dec_digit(c) && c != '.').verify_map(|s| {
                      let mut nested_input = ConundrumInput {
                input: s,
                state: Arc::new(RwLock::new(ParseState {
                    data: MdxParsingResult::from_initial_mdx_content(s),
..Default::default()
                }))
            };
                      let nested_checkpoint = nested_input.input.checkpoint();
                      let n_int = conundrum_int.parse_next(&mut nested_input);
                      if n_int.is_ok() && nested_input.input.is_empty() {
                          return n_int.ok();
                      } else {
                          nested_input.input.reset(&nested_checkpoint);
                      }
                      let n_float = conundrum_float.parse_next(&mut nested_input);
                      if nested_input.input.is_empty() {
                          n_float.ok()
                      } else {
                          None
                      }
                  })
                  .parse_next(input)
                  .inspect_err(|_| {
                      input.input.reset(&start);
                  })?;
        Ok(res)
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

    pub fn is_float_and(&self, evaluate: impl Fn(&ConundrumFloat) -> bool) -> bool {
        match self {
            ConundrumNumber::Int(_) => false,
            ConundrumNumber::Float(n) => evaluate(n),
        }
    }

    pub fn is_int_and(&self, evaluate: impl Fn(&ConundrumInt) -> bool) -> bool {
        match self {
            ConundrumNumber::Float(_) => false,
            ConundrumNumber::Int(n) => evaluate(n),
        }
    }
}

impl ConundrumComponentResult for ConundrumNumber {
    fn to_conundrum_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_int() {
        let mut input = wrap_test_conundrum_content("31415");
        let res = ConundrumNumber::parse_conundrum(&mut input).expect("Parses a valid number");
        assert!(matches!(res, ConundrumNumber::Int(_)), "Finds an int when one is present.");
        assert!(res.is_int_and(|n| *n == 31415), "Finds the proper integer value");
    }
    #[test]
    fn parses_floats() {
        let mut input = wrap_test_conundrum_content("3.1415");
        let res = ConundrumNumber::parse_conundrum(&mut input).expect("Parses a valid number");
        println!("Res: {:#?}", res);
        assert!(matches!(res, ConundrumNumber::Float(_)), "Finds a float when one is present.");
        #[allow(clippy::approx_constant)]
        assert!(res.is_float_and(|n| *n == 3.1415), "Finds the proper float value");
    }
}

use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use winnow::{Parser, ascii::dec_int, combinator::opt, error::ContextError, stream::Stream};

use crate::lang::runtime::{
    state::conundrum_error_variant::ConundrumModalResult,
    traits::{
        conundrum_input::{ArcState, ConundrumInput},
        html_js_component_result::HtmlJsComponentResult,
    },
};

/// If only 2 components are passed, as in `[My link](video:myId@4:30)`, it is
/// assumed to be minutes and seconds. If three components are passed, it will
/// be applied as `minutes:seconds:hours`
#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Timestamp {
    pub min: i32,
    pub hours: Option<i32>,
    pub sec: i32,
}

pub fn timestamp<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<Timestamp> {
    let start = input.input.checkpoint();

    let hours = dec_int.parse_next(input).inspect_err(|_| {
                                              input.input.reset(&start);
                                          })?;

    ':'.parse_next(input).inspect_err(|_| {
                              input.input.reset(&start);
                          })?;
    let min = dec_int.parse_next(input).inspect_err(|_| {
                                            input.input.reset(&start);
                                        })?;

    if let Some(sec) = opt(|nested_input: &mut ConundrumInput<'a>| {
                           let nested_start = nested_input.input.checkpoint();
                           ':'.parse_next(nested_input).inspect_err(|_| {
                                                            nested_input.input.reset(&nested_start);
                                                        })?;
                           dec_int.parse_next(nested_input).inspect_err(|_: &ContextError| {
                                                               nested_input.input.reset(&nested_start);
                                                           })
                       }).parse_next(input)
                         .ok()
                         .flatten()
    {
        Ok(Timestamp { hours: Some(hours),
                       min,
                       sec })
    } else {
        Ok(Timestamp { min: hours,
                       sec: min,
                       hours: None })
    }
}

impl HtmlJsComponentResult for Timestamp {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}

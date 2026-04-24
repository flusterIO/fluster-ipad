use winnow::{
    Parser,
    combinator::{alt, delimited},
    token::take_till,
};

use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    parsers::conundrum::logic::string::conundrum_string::ConundrumString,
};

pub fn double_quoted_javascript_string(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumString> {
    let res = delimited('"', take_till(0.., |c| c == '\n' || c == '"'), '"').parse_next(input)?;
    Ok(ConundrumString(res.to_string()))
}

pub fn single_quoted_javascript_string(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumString> {
    let res = delimited('\'', take_till(0.., |c| c == '\n' || c == '\''), '\'').parse_next(input)?;
    Ok(ConundrumString(res.to_string()))
}

pub fn back_tick_quoted_javascript_string(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumString> {
    let res = delimited('`', take_till(0.., |c| c == '`'), '`').parse_next(input)?;
    Ok(ConundrumString(res.to_string()))
}

pub fn single_or_double_quoted_string(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumString> {
    alt((single_quoted_javascript_string, double_quoted_javascript_string)).parse_next(input)
}

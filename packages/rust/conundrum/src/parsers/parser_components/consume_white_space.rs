use winnow::{
    Parser,
    stream::{AsChar, Range},
    token::take_while,
};

use crate::lang::runtime::{
    state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput,
};

pub fn consume_white_space(occurrences: impl Into<Range> + Clone)
                           -> impl Fn(&mut ConundrumInput) -> ConundrumModalResult<()> {
    move |input| {
        take_while(occurrences.clone(), |c| AsChar::is_space(c) || AsChar::is_newline(c)).parse_next(input)?;
        Ok(())
    }
}

/// Consumes spaces and tabs, but does fails on new lines.
pub fn consume_linear_space(occurrences: impl Into<Range> + Clone)
                            -> impl Fn(&mut ConundrumInput) -> ConundrumModalResult<()> {
    move |input| {
        take_while(occurrences.clone(), AsChar::is_space).void().parse_next(input)?;
        Ok(())
    }
}

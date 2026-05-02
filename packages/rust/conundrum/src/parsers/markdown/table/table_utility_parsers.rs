use crate::lang::runtime::{
    state::{
        conundrum_error::ConundrumError,
        conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
    },
    traits::conundrum_input::ConundrumInput,
};
use winnow::{
    Parser,
    combinator::not,
    error::{ContextError, ErrMode},
};

pub fn table_separator_not_escaped(input: &mut ConundrumInput) -> ConundrumModalResult<()> {
    (not('\\'), '|').void()
        .map_err(|_: ContextError| {
            ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Not a table separator.")))
        })
        .parse_next(input)
}

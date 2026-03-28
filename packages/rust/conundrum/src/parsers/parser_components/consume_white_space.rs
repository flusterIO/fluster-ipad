use winnow::{
    ModalResult, Parser,
    stream::{AsChar, Range},
    token::take_while,
};

use crate::lang::runtime::traits::conundrum_input::ConundrumInput;

pub fn consume_white_space(occurrences: impl Into<Range> + Clone) -> impl Fn(&mut ConundrumInput) -> ModalResult<()> {
    move |input| {
        take_while(occurrences.clone(), AsChar::is_space).parse_next(input)?;
        Ok(())
    }
}

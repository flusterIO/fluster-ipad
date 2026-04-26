use crate::lang::runtime::{
    state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput,
};
use winnow::{
    Parser,
    stream::{AsChar, Range},
    token::take_while,
};

pub fn space_of_length(occurences: impl Into<Range> + Clone)
                       -> impl Fn(&mut ConundrumInput) -> ConundrumModalResult<String> {
    move |input: &mut ConundrumInput| {
        take_while(occurences.clone(), AsChar::is_space).map(String::from).parse_next(input)
    }
}

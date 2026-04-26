use crate::lang::runtime::{
    state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput,
};
use winnow::{Parser, combinator::repeat, stream::Range};

pub fn repeat_with_count<'a, T>(occurrences: impl Into<Range> + Clone,
                                mut parser: impl Fn(&mut ConundrumInput<'a>) -> ConundrumModalResult<T>)
                                -> impl FnMut(&mut ConundrumInput<'a>) -> ConundrumModalResult<(Vec<T>, usize)> {
    move |input: &mut ConundrumInput<'a>| {
        let mut count: usize = 0;
        let res: Vec<T> = repeat(occurrences.clone(), |nested_input: &mut ConundrumInput<'a>| {
                              let nested_res: T = parser.parse_next(nested_input)?;
                              count += 1;
                              Ok(nested_res)
                          }).parse_next(input)?;

        Ok((res, count))
    }
}

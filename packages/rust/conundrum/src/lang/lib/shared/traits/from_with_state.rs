use crate::lang::runtime::state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState};

pub trait FromWithState<T> {
    fn from_with_state(value: T, state: &mut ParseState) -> ConundrumModalResult<Self>
        where Self: Sized;
}

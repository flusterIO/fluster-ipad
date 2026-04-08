use crate::lang::runtime::state::parse_state::ParseState;

pub trait FromWithState<T> {
    fn from_with_state(value: T, state: &mut ParseState) -> Self;
}

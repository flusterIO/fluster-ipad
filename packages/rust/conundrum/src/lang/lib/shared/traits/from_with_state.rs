use crate::lang::runtime::{
    state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState},
    traits::conundrum_input::ArcState,
};

pub trait FromWithState<T> {
    fn from_with_state(value: T, state: ArcState) -> ConundrumModalResult<Self>
        where Self: Sized;
}

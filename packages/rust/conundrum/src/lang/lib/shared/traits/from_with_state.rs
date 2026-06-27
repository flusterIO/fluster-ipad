use crate::lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ArcState};

pub trait FromWithState<T> {
    fn from_with_state(value: T, state: ArcState) -> ConundrumModalResult<Self>
        where Self: Sized;
}

pub trait FromWithParam<T, J> {
    fn from_with_param(value: T, param: J) -> Self
        where Self: Sized;
}

pub trait TryFromWithParam<T, J> {
    fn try_from_with_param(value: T, param: J) -> ConundrumModalResult<Self>
        where Self: Sized;
}

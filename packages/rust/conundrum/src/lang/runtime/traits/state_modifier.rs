use crate::lang::runtime::{state::parse_state::ParseState, traits::conundrum_input::ArcState};

/// Not all components will modify state, but those that do should implement
/// this trait.
pub trait ConundrumStateModifier<T> {
    /// Takes the current `ParseState` and modifies it according to the
    /// component's properties.
    fn set_state(res: ArcState, data: Option<T>);
}

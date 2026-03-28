use crate::lang::runtime::state::parse_state::ParseState;

/// Not all components will modify state, but those that do should implement
/// this trait.
pub trait ConundrumStateModifier {
    /// Takes the current `ParseState` and modifies it according to the
    /// component's properties.
    fn set_state(&self, res: &mut ParseState);
}

use crate::lang::runtime::state::parse_state::ParseState;

pub trait AIInputComponentResult {
    /// The primary output method for Fluster when using the modifier flags to
    /// dynamically set the output format.
    fn to_ai_input(&self, res: &mut ParseState) -> String;
}

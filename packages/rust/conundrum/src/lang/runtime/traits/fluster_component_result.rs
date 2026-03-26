use crate::lang::runtime::state::parse_state::ParseState;

pub trait FlusterComponentResult {
    /// The primary output method for Fluster when using the modifier flags to
    /// dynamically set the output format.
    fn to_fluster_component(&self, res: &mut ParseState) -> String;
}

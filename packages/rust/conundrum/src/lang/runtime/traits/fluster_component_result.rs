use crate::lang::runtime::state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState};

pub trait ConundrumComponentResult {
    /// The primary output method for Fluster when using the modifier flags to
    /// dynamically set the output format.
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String>;
}

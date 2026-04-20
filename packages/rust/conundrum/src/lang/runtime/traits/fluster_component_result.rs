use crate::lang::runtime::{
    state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState},
    traits::conundrum_input::ArcState,
};

pub trait ConundrumComponentResult {
    /// The primary output method for Fluster when using the modifier flags to
    /// dynamically set the output format.
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String>;
}

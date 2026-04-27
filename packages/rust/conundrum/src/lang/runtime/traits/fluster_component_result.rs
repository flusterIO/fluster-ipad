use crate::lang::runtime::{
    state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState},
    traits::conundrum_input::ArcState,
};

/// Sort of a decision maker on the component level, this trait most often
/// doesn't output anything unique, but rather decides which other output to use
/// based on the state of the compiled appliication (different flags, compile
/// target, presence of other components, etc...);
pub trait ConundrumComponentResult {
    /// The primary output method for Fluster when using the modifier flags to
    /// dynamically set the output format.
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String>;
}

use crate::lang::runtime::{
    state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState},
    traits::conundrum_input::ArcState,
};

pub trait FullTextSearchInputComponentResult {
    /// Useful mostly for searchability. Formatting may be hideous.
    fn to_full_text_search_input(&self, res: ArcState) -> ConundrumModalResult<String>;
}

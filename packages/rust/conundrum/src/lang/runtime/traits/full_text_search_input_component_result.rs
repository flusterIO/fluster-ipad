use crate::lang::runtime::state::parse_state::ParseState;

pub trait FullTextSearchInputComponentResult {
    /// Useful mostly for searchability. Formatting may be hideous.
    fn to_full_text_search_input(&self, res: &mut ParseState) -> String;
}

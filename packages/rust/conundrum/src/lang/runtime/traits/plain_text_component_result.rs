use crate::lang::runtime::state::{conundrum_error_variant::ConundrumModalResult, parse_state::ParseState};

pub trait PlainTextComponentResult {
    /// Useful mostly for searchability, so markdown mdx or Conundrum syntax
    /// doesn't interfere with the text matching algorithm. Formatting may
    /// be hideous.
    fn to_plain_text(&self, res: &mut ParseState) -> ConundrumModalResult<String>;
}

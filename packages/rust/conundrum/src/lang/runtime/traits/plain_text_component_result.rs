use crate::lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ArcState};

pub trait PlainTextComponentResult {
    /// Useful mostly for searchability, so markdown mdx or Conundrum syntax
    /// doesn't interfere with the text matching algorithm. Formatting may
    /// be hideous.
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String>;
}

use crate::lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ArcState};

/// For then the output is inntended to be consumed by AI, some components may
/// have the ability to provide additional useful context to the model, and more
/// importantly, all components have the ability to output to markdown... a
/// format models consume frequently and arguably the format that they handle
/// best, in general.
pub trait AIInputComponentResult {
    /// The primary output method for Fluster when using the modifier flags to
    /// dynamically set the output format.
    fn to_ai_input(&self, res: ArcState) -> ConundrumModalResult<String>;
}

use askama::Template;

use crate::lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ArcState};

/// When a struct can be represented as a ConundrumTemplate, this returns the
/// struct for a uniform way to gather them, allowing some manipulation of the
/// properties in some edge cases before being actually rendered.
pub trait AsTemplate<T: Template> {
    fn as_template(&self, state: ArcState) -> ConundrumModalResult<T>;
}

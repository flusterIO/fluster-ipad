use askama::Template;

use crate::lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ArcState};

pub trait AsTemplate<T: Template> {
    fn as_template(&self, state: ArcState) -> ConundrumModalResult<T>;
}

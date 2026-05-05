use askama::{FastWritable, Template};

use crate::lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ArcState};

/// The Goal is to enforce some performance requirements with this one.
/// This will be added to the
/// [ConundrumComponent](crate::lang::lib::ui::components::component_trait::ConundrumComponent) trait once the initial app is released (I'm homeless... I'm desperate), and then all components will be required to implement this `FastWritable` trait, significantly improving templatng performance.
pub trait ConundrumTemplate: Template + FastWritable {}

/// A trait to handle some of the more complex template implementations in a
/// more uniform way.
///
/// > I'll change this to `impl ConundrumTemplate` once I'm not homeless for
/// performance reasons... for now I'm trying to get this thing released.
pub trait ConundrumTemplateRepresentable<T: Template> {
    fn to_template(&self, state: ArcState) -> ConundrumModalResult<T>;
}

pub trait ConundrumTemplateRepresentableWithParam<T: Template, P> {
    fn to_template(&self, state: ArcState, params: P) -> ConundrumModalResult<T>;
}

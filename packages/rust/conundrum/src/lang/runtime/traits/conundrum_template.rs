use askama::{FastWritable, Template};

use crate::lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ArcState};

/// The Goal is to enforce some performance requirements with this one.
/// This will be added to the
/// [ConundrumComponent](crate::lang::lib::ui::components::component_trait::ConundrumComponent) trait once the initial app is released (I'm homeless... I'm desperate), and then all components will be required to implement this `FastWritable` trait, significantly improving templatng performance.
pub trait ConundrumTemplate: Template + FastWritable {}

pub trait CDRMTemplateRepresentable<T: Template> {
    fn as_cdrm_template(&self) -> T;
}

pub trait CDRMTemplateRepresentableWithParam<T: Template, P> {
    fn as_cdrm_template(&self, param: P) -> T;
}

pub trait CDRMTemplatePossiblyRepresentable<T: Template> {
    fn to_cdrm_template(&self, state: ArcState) -> ConundrumModalResult<T>;
}

pub trait HTMLTemplateRepresentable<T: Template> {
    fn as_template(&self) -> T;
}

pub trait HTMLTemplateVariantRepresentable<T: Template, V> {
    fn as_template_variant(&self, variant: V) -> T;
}

/// A trait to handle some of the more complex template implementations in a
/// more uniform way.
///
/// > I'll change this to `impl ConundrumTemplate` once I'm not homeless for
/// performance reasons... for now I'm trying to get this thing released.
pub trait HTMLTemplatePossiblyRepresentable<T: Template> {
    fn to_template(&self, state: ArcState) -> ConundrumModalResult<T>;
}

pub trait HTMLTemplatePossiblyRepresentableWithParam<T: Template, P> {
    fn to_template(&self, state: ArcState, params: P) -> ConundrumModalResult<T>;
}

use askama::{FastWritable, Template};

/// The Goal is to enforce some performance requirements with this one.
/// This will be added to the
/// [ConundrumComponent](crate::lang::lib::ui::components::component_trait::ConundrumComponent) trait once the initial app is released (I'm homeless... I'm desperate), and then all components will be required to implement this `FastWritable` trait, significantly improving templatng performance.
pub trait ConundrumTemplate: Template + FastWritable {}

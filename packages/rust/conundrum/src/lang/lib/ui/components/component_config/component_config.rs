use crate::{
    lang::lib::ui::components::component_config::{component_docs::ComponentDocs, component_kind::ComponentKind},
    output::html::glue::component_glue_manager::AnyComponentKey,
};

/// # ComponentConfig
///
/// This is all still held by typescript, but everything will be making it's way
/// to Rust so that we can more easily expose the docs to AI.
pub struct ComponentConfig {
    pub name: AnyComponentKey,
    pub component_type: ComponentKind,
    pub docs: ComponentDocs,
}

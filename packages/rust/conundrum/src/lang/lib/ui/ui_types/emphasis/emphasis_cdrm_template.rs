use askama::Template;

use crate::{
    lang::lib::ui::{shared_props::sizable::SizablePropsGroup, ui_types::emphasis::emphasis_model::Emphasis},
    output::general::component_constants::documentation_component_name::DocumentationComponentName,
};

/// ## Template (HTML)
/// ```askama
/// <{{DocumentationComponentName::Emphasis}} {{self.emphasis}} />
/// ```
#[derive(Template)]
#[template(in_doc = true, ext = "html")]
pub struct CDRMEmphasisTemplate {
    pub emphasis: Emphasis,
    pub sizable: SizablePropsGroup,
}

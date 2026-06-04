use askama::Template;
use conundrum_macro_traits::conundrum_macro::ConundrumPropertyMap;

use crate::{
    lang::lib::ui::{shared_props::sizable::SizablePropsGroup, ui_types::emphasis::emphasis_model::Emphasis},
    output::general::component_constants::{
        component_names::EmbeddableComponentName,
    },
};

/// ## Template (HTML)
/// ```askama
/// <{{EmbeddableComponentName::Color | safe}} {{self.emphasis | safe}} {{self.sizable.to_cdrm_property_stream() | safe}} />
/// ```
#[derive(Template)]
#[template(in_doc = true, ext = "html")]
pub struct CDRMEmphasisTemplate {
    pub emphasis: Emphasis,
    pub sizable: SizablePropsGroup,
}

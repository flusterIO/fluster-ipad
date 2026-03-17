use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use typeshare::typeshare;

use crate::core_types::component_constants::component_ids::EmbeddableComponentId;

#[typeshare]
#[derive(Display, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmbeddableComponentName {
    #[serde(rename = "Admonition")]
    #[strum(to_string = "Admonition")]
    Admonition,
    #[serde(rename = "Hl")]
    #[strum(to_string = "Hl")]
    Hl,
    #[serde(rename = "Highlight")]
    #[strum(to_string = "Highlight")]
    Highlight,
    #[serde(rename = "Ul")]
    #[strum(to_string = "Ul")]
    Ul,
    #[serde(rename = "Underline")]
    #[strum(to_string = "Underline")]
    Underline,
    #[serde(rename = "Card")]
    #[strum(to_string = "Card")]
    Card,
    #[serde(rename = "Grid")]
    #[strum(to_string = "Grid")]
    Grid,
    #[serde(rename = "Container")]
    #[strum(to_string = "Container")]
    UtlityContainer,
    #[serde(rename = "Hr")]
    #[strum(to_string = "Hr")]
    HrWithChildren,
    #[serde(rename = "Hint")]
    #[strum(to_string = "Hint")]
    Hint,
}

impl EmbeddableComponentName {
    pub fn to_component_id(&self) -> EmbeddableComponentId {
        match self {
            EmbeddableComponentName::UtlityContainer => EmbeddableComponentId::UtlityContainer,
            EmbeddableComponentName::Hint => EmbeddableComponentId::Hint,
            EmbeddableComponentName::HrWithChildren => EmbeddableComponentId::HrWithChildren,
            EmbeddableComponentName::Grid => EmbeddableComponentId::Grid,
            EmbeddableComponentName::Underline => EmbeddableComponentId::Ul,
            EmbeddableComponentName::Ul => EmbeddableComponentId::Ul,
            EmbeddableComponentName::Hl => EmbeddableComponentId::Hl,
            EmbeddableComponentName::Highlight => EmbeddableComponentId::Hl,
            EmbeddableComponentName::Admonition => EmbeddableComponentId::Admonition,
            EmbeddableComponentName::Card => EmbeddableComponentId::Card,
        }
    }
}

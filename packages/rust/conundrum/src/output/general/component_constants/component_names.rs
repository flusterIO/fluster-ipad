use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use typeshare::typeshare;

use crate::{
    lang::runtime::state::{conundrum_error::ConundrumError, conundrum_error_variant::ConundrumErrorVariant},
    output::general::component_constants::component_ids::EmbeddableComponentId,
};

#[typeshare]
#[derive(Display, uniffi::Enum, Debug, Clone, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize)]
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
    #[serde(rename = "EqRef")]
    #[strum(to_string = "EqRef")]
    EqRef,
    #[serde(rename = "Tabs")]
    #[strum(to_string = "Tabs")]
    Tabs,
    #[serde(rename = "Tab")]
    #[strum(to_string = "Tab")]
    Tab,
    #[serde(rename = "AINoteSummary")]
    #[strum(to_string = "AINoteSummary")]
    AINoteSummary,
}

impl FromStr for EmbeddableComponentName {
    type Err = ConundrumErrorVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EmbeddableComponentName::iter().find(|item| {
            item.to_string().as_str() == s
        }).ok_or(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid component name", "Conundrum found a component name that doesn't exist It looks like you might be using a valid syntax with a component name that's not available. See `Components??` for the documentation of the available components.")))
    }
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
            EmbeddableComponentName::AINoteSummary => EmbeddableComponentId::AINoteSummary,
            EmbeddableComponentName::Tabs => EmbeddableComponentId::Tabs,
            EmbeddableComponentName::Tab => EmbeddableComponentId::Tab,
            EmbeddableComponentName::EqRef => EmbeddableComponentId::EqRef,
        }
    }
}

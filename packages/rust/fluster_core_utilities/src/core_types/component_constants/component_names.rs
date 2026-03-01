use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use typeshare::typeshare;

#[typeshare]
#[derive(Display, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmbeddableComponentName {
    #[serde(rename = "Admonition")]
    #[strum(to_string = "Admonition")]
    Admonition,
    #[serde(rename = "Hl")]
    #[strum(to_string = "Hl")]
    Hl,
    #[serde(rename = "HL")]
    #[strum(to_string = "HL")]
    HL,
    #[serde(rename = "Ul")]
    #[strum(to_string = "Ul")]
    Ul,
    #[serde(rename = "UL")]
    #[strum(to_string = "UL")]
    UL,
    #[serde(rename = "Card")]
    #[strum(to_string = "Card")]
    Card,
    #[serde(rename = "Grid")]
    #[strum(to_string = "Grid")]
    Grid,
    #[serde(rename = "Container")]
    #[strum(to_string = "Container")]
    UtlityContainer,
}

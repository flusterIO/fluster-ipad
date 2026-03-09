use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, strum_macros::Display, Serialize, Deserialize)]
pub enum FlusterTheme {
    #[serde(rename = "zinc")]
    #[strum(to_string = "zinc")]
    Zinc,
    #[serde(rename = "yellow")]
    #[strum(to_string = "yellow")]
    Yellow,
    #[serde(rename = "violet")]
    #[strum(to_string = "violet")]
    Violet,
    #[serde(rename = "fluster")]
    #[strum(to_string = "fluster")]
    Fluster,
    #[serde(rename = "stone")]
    #[strum(to_string = "stone")]
    Stone,
    #[serde(rename = "slate")]
    #[strum(to_string = "slate")]
    Slate,
    #[serde(rename = "rose")]
    #[strum(to_string = "rose")]
    Rose,
    #[serde(rename = "red")]
    #[strum(to_string = "red")]
    Red,
    #[serde(rename = "orange")]
    #[strum(to_string = "orange")]
    Orange,
    #[serde(rename = "neutral")]
    #[strum(to_string = "neutral")]
    Neutral,
    #[serde(rename = "green")]
    #[strum(to_string = "green")]
    Green,
    #[serde(rename = "gray")]
    #[strum(to_string = "gray")]
    Gray,
    #[serde(rename = "blue")]
    #[strum(to_string = "blue")]
    Blue,
}

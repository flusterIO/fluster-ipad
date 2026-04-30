use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct EquationReferenceRecord {}

#[typeshare]
#[derive(uniffi::Enum, Serialize, Deserialize, strum_macros::Display)]
pub enum EquationNumberingStrategy {
    #[serde(rename = "none")]
    #[strum(to_string = "none")]
    None,
    #[serde(rename = "id-only")]
    #[strum(to_string = "id-only")]
    IdOnly,
    #[serde(rename = "all")]
    #[strum(to_string = "all")]
    All,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct MathState {
    pub mathjax_font_url: String,
    pub equation_refs: HashMap<String, u32>,
    pub hide_equation_labels: EquationNumberingStrategy,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct InitialMathState {
    pub mathjax_font_url: String,
    pub hide_equation_labels: EquationNumberingStrategy,
}

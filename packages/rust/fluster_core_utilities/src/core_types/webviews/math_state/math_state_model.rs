use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct EquationReferenceRecord {}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct MathState {
    pub mathjax_font_url: String,
    pub equation_refs: HashMap<String, u32>,
    pub hide_equation_labels: bool,
}

#[typeshare]
#[derive(uniffi::Record, Serialize, Deserialize)]
pub struct InitialMathState {
    pub mathjax_font_url: String,
}

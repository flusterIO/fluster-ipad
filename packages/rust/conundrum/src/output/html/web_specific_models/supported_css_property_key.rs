use crate::{
    lang::runtime::state::conundrum_error_variant::ConundrumModalResult,
    output::html::web_specific_traits::to_string_react_casing::ToStringReactCasing,
};

/// Maps to css property keys, not the tailwind equivalents that I used way too
/// much...
#[derive(Eq, PartialEq, Hash, strum_macros::Display, serde::Serialize, serde::Deserialize)]
pub enum CssPropertyKey {
    #[serde(rename = "color")]
    #[strum(to_string = "color")]
    Color,
    #[serde(rename = "background-color")]
    #[strum(to_string = "background-color")]
    BackgroundColor,
    #[serde(rename = "fill")]
    #[strum(to_string = "fill")]
    Fill,
    #[serde(rename = "stroke")]
    #[strum(to_string = "stroke")]
    Stroke,
}

impl ToStringReactCasing for CssPropertyKey {
    fn to_string_react_casing(&self) -> ConundrumModalResult<String> {
        Ok(match self {
               Self::BackgroundColor => "backgroundColor",
               Self::Stroke => "stroke",
               Self::Fill => "fill",
               Self::Color => "color",
           }.to_string())
    }
}

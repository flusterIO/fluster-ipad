use askama::filters::HtmlSafe;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use typeshare::typeshare;
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::ui::{
            ui_traits::to_text_representation::ToTextRepresentation,
            ui_types::emphasis::variable_to_emphasis::css_variable_to_emphasis,
        },
        runtime::state::conundrum_error_variant::ConundrumErrorVariant,
    },
    output::html::web_specific_traits::css_value_representable::CSSValueRepresentable,
};

#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CSSColorVariable(pub String);

impl HtmlSafe for CSSColorVariable {}

impl ToTextRepresentation for CSSColorVariable {
    fn to_text_repr(&self) -> String {
        css_variable_to_emphasis(self.0.as_str()).map(|item| item.to_text_repr()).unwrap_or_else(|| self.0.clone())
    }
}

impl Default for CSSColorVariable {
    fn default() -> Self {
        Self(String::from("--color-primary"))
    }
}

impl TryFrom<String> for CSSColorVariable {
    type Error = ErrMode<ConundrumErrorVariant>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("--") {
            Ok(Self(value))
        } else {
            Err(ErrMode::Backtrack(ConundrumErrorVariant::InvalidCSSVariableSyntax(value)))
        }
    }
}

impl Display for CSSColorVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CSSValueRepresentable for CSSColorVariable {
    fn as_css_value(&self) -> String {
        format!("var({})", self.0)
    }
}

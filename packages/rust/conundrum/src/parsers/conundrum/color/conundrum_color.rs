use std::fmt::Display;

use lightningcss::traits::ToCss;
use serde::Serialize;
use typeshare::typeshare;
use winnow::{Parser, error::ErrMode};

use crate::{
    lang::{
        lib::ui::ui_types::emphasis::emphasis_model::Emphasis,
        runtime::{
            state::conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            traits::conundrum_input::ConundrumInput,
        },
    },
    output::html::{
        web_specific_models::lightning_css_printer_options::safari_specific_lightning_css_printer_options,
        web_specific_traits::css_value_representable::{CSSInlineHtmlValuePairRepresentable, CSSValueRepresentable},
    },
    parsers::{
        conundrum::{
            color::{color_pair::ColorPair, css_color::CssColor, css_color_variable::CSSColorVariable},
            logic::string::conundrum_string::ConundrumString,
        },
        parser_trait::ConundrumParser,
    },
};

#[typeshare]
#[derive(Debug, Serialize, serde::Deserialize, Clone)]
pub enum ConundrumColor {
    Emphasis(Emphasis),
    Css(CssColor),
    /// Accepts a css variable string alone, not wrapped in the `var` syntax.
    /// ## Exmample
    ///
    /// ```rs
    /// ConundrumColor::CSSVariable(String::from("--color-emphasis-warn"))
    /// ```
    CSSVariable(CSSColorVariable),
}

impl Display for ConundrumColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConundrumColor::Emphasis(e) => e.fmt(f),
            ConundrumColor::CSSVariable(c) => c.fmt(f),
            ConundrumColor::Css(c) => c.fmt(f),
        }
    }
}

impl ConundrumColor {
    pub fn describe(&self) -> String {
        match self {
            Self::Emphasis(e) => e.to_string(),
            Self::Css(c) => {
                c.0.to_css_string(safari_specific_lightning_css_printer_options()).unwrap_or_else(|_| "--".to_string())
            }
            Self::CSSVariable(c) => c.to_string(),
        }
    }

    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        ConundrumColor::Css(CssColor::from_rgba(red, green, blue, alpha))
    }
}

impl TryFrom<ConundrumString> for ConundrumColor {
    type Error = ErrMode<ConundrumErrorVariant>;

    fn try_from(value: ConundrumString) -> Result<Self, Self::Error> {
        CssColor::try_from(value.0.as_str()).map(|c| ConundrumColor::Css(c))
    }
}

impl CSSValueRepresentable for ConundrumColor {
    fn as_css_value(&self) -> String {
        match self {
            Self::Css(x) => x.as_css_value(),
            Self::Emphasis(e) => e.as_inline_style_value_group().background.as_css_value(),
            Self::CSSVariable(v) => v.as_css_value(),
        }
    }
}

impl CSSInlineHtmlValuePairRepresentable<ConundrumColor> for ConundrumColor {
    fn as_inline_style_value_group(&self) -> super::color_pair::ColorPair<ConundrumColor> {
        match self {
            Self::Emphasis(e) => e.as_inline_style_value_group(),
            Self::Css(c) => c.as_inline_style_value_group(),
            Self::CSSVariable(c) => {
                let v = ConundrumColor::CSSVariable(c.clone());
                ColorPair { // TODO: Derive the foreground color here.
                            foreground: v.clone(),
                            background: v }
            }
        }
    }
}

impl ConundrumParser<ConundrumColor> for ConundrumColor {
    fn parse_input_string(input: &mut ConundrumInput) -> ConundrumModalResult<ConundrumColor> {
        if let Ok(emphasis) = Emphasis::parse_input_string.parse_next(input) {
            Ok(ConundrumColor::Emphasis(emphasis))
        } else if let Ok(css_color) = CssColor::parse_input_string.parse_next(input) {
            Ok(ConundrumColor::Css(css_color))
        } else {
            Err(ErrMode::Backtrack(ConundrumErrorVariant::InvalidColor("unknown".to_string())))
        }
    }

    fn matches_first_char(_: char) -> bool {
        true
    }
}

use ratatui::text::ToSpan;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};
use strum_macros::{Display, EnumString};
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::shared::traits::from_with_state::TryFromWithParam,
        runtime::state::conundrum_error_variant::ConundrumErrorVariant,
    },
    parsers::{
        conundrum::logic::string::conundrum_string::ConundrumString,
        markdown::code_block::mermaid::mermaid_theme::MermaidTheme,
    },
};
#[typeshare::typeshare]
#[derive(Serialize, Deserialize, Display, EnumString, EnumIter, uniffi::Enum, Clone, Default, Debug, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum SupportedCodeBlockTheme {
    #[serde(rename = "1337")]
    #[strum(to_string = "1337")]
    ThirteenThirtySeven,
    #[serde(rename = "Coldark-Cold")]
    #[strum(to_string = "Coldark-Cold")]
    ColdarkCold,
    #[serde(rename = "Coldark-Dark")]
    #[strum(to_string = "Coldark-Dark")]
    ColdarkDark,
    #[serde(rename = "DarkNeon")]
    #[strum(to_string = "DarkNeon")]
    Darkneon,
    #[serde(rename = "Dracula")]
    #[strum(to_string = "Dracula")]
    #[default]
    Dracula,
    #[serde(rename = "GitHub")]
    #[strum(to_string = "GitHub")]
    Github,
    #[serde(rename = "Monokai Extended")]
    #[strum(to_string = "Monokai Extended")]
    MonokaiExtended,
    #[serde(rename = "Monokai Extended Bright")]
    #[strum(to_string = "Monokai Extended Bright")]
    MonokaiExtendedBright,
    #[serde(rename = "Monokai Extended Light")]
    #[strum(to_string = "Monokai Extended Light")]
    MonokaiExtendedLight,
    #[serde(rename = "Monokai Extended Origin")]
    #[strum(to_string = "Monokai Extended Origin")]
    MonoakaiExtendedOrigin,
    #[serde(rename = "Nord")]
    #[strum(to_string = "Nord")]
    Nord,
    #[serde(rename = "OneHalfDark")]
    #[strum(to_string = "OneHalfDark")]
    Onehalfdark,
    #[serde(rename = "OneHalfLight")]
    #[strum(to_string = "OneHalfLight")]
    Onehalflight,
    #[serde(rename = "Solarized (dark)")]
    #[strum(to_string = "Solarized (dark)")]
    SolarizedDark,
    #[serde(rename = "Solarized (light)")]
    #[strum(to_string = "Solarized (light)")]
    SolarizedLight,
    #[serde(rename = "Sublime Snazzy")]
    #[strum(to_string = "Sublime Snazzy")]
    SublimeSnazzy,
    #[serde(rename = "TwoDark")]
    #[strum(to_string = "TwoDark")]
    Twodark,
    #[serde(rename = "Visual Studio Dark+")]
    #[strum(to_string = "Visual Studio Dark+")]
    VisualStudioDarkPlus,
    #[serde(rename = "ansi")]
    #[strum(to_string = "ansi")]
    Ansi,
    #[serde(rename = "base16")]
    #[strum(to_string = "base16")]
    Base16,
    #[serde(rename = "base16-256")]
    #[strum(to_string = "base16-256")]
    Base16_256,
    #[serde(rename = "gruvbox-dark")]
    #[strum(to_string = "gruvbox-dark")]
    GruvboxDark,
    #[serde(rename = "gruvbox-light")]
    #[strum(to_string = "gruvbox-light")]
    GruvboxLight,
    #[serde(rename = "zenburn")]
    #[strum(to_string = "zenburn")]
    Zenburn,
}

impl SupportedCodeBlockTheme {
    /// ## TODO:
    /// - [ ] Actually test all hese f--king themes to make sure they make sense
    ///   with the them that
    /// they're paired with. Going off of names isn't enough, but I'm in a
    /// hurry...
    pub fn to_mermaid_theme(&self, dark_mode: bool) -> MermaidTheme {
        match self {
            Self::GruvboxLight => MermaidTheme::ZincLight,
            Self::Base16_256 => MermaidTheme::ZincDark,
            Self::Darkneon => MermaidTheme::TokyoNight,
            Self::MonokaiExtendedBright => MermaidTheme::TokyoNightStorm,
            Self::MonokaiExtended => MermaidTheme::TokyoNightLight,
            Self::SublimeSnazzy => match dark_mode {
                true => MermaidTheme::CatppuccinLatte,
                false => MermaidTheme::CatppuccinMocha,
            },
            Self::Nord => match dark_mode {
                true => MermaidTheme::Nord,
                false => MermaidTheme::NordLight,
            },
            Self::Dracula => MermaidTheme::Dracula,
            Self::Github => match dark_mode {
                true => MermaidTheme::GithubDark,
                false => MermaidTheme::GithubLight,
            },
            Self::SolarizedLight => MermaidTheme::SolarizedLight,
            Self::SolarizedDark => MermaidTheme::SolarizedDark,
            Self::Onehalfdark => MermaidTheme::OneDark,
            _ => MermaidTheme::Auto,
        }
    }
}

impl TryFrom<ConundrumString> for SupportedCodeBlockTheme {
    type Error = ErrMode<ConundrumErrorVariant>;

    fn try_from(value: ConundrumString) -> Result<Self, Self::Error> {
        for f in SupportedCodeBlockTheme::iter() {
            if f.to_string() == value.0 {
                return Ok(f);
            }
        }
        Err(ErrMode::Backtrack(ConundrumErrorVariant::TypeConversionError { from: String::from("string"),
                                                                            to: String::from("SupportedCodeBlockTheme") }))
    }
}

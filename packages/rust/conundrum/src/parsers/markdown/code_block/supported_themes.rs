use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum_macros::{Display, EnumString};
use syntect::highlighting::{Theme, ThemeSet};

#[typeshare::typeshare]
#[derive(Serialize, Deserialize, Display, EnumString, EnumIter, uniffi::Enum, Clone, Default, Debug)]
#[allow(non_camel_case_types)]
pub enum SupportedCodeBlockTheme {
    #[serde(rename = "InspiredGitHub", alias = "github")]
    #[strum(to_string = "InspiredGitHub")]
    InspiredGitHub,
    #[serde(rename = "Solarized (dark)", alias = "solarized-dark", alias = "solarizeddark")]
    #[strum(to_string = "Solarized (dark)")]
    #[default]
    SolarizedDark,
    #[serde(rename = "Solarized (light)", alias = "solarized-light", alias = "solarizedlight")]
    #[strum(to_string = "Solarized (light)")]
    SolarizedLight,
    #[serde(rename = "base16-eighties.dark", alias = "80sdark", alias = "80s-dark")]
    #[strum(to_string = "base16-eighties.dark")]
    Base16_80sDark,
    #[serde(rename = "base16-mocha.dark", alias = "mocha-dark", alias = "mochadark")]
    #[strum(to_string = "base16-mocha.dark")]
    #[allow(non_camel_case_types)]
    Base16_MochaDark,
    #[serde(rename = "base16-ocean.dark", alias = "ocean-dark", alias = "oceandark")]
    #[strum(to_string = "base16-ocean.dark")]
    #[allow(non_camel_case_types)]
    Base16_OceanDark,
    #[serde(rename = "base16-ocean.light", alias = "ocean-light", alias = "oceanlight")]
    #[strum(to_string = "base16-ocean.light")]
    Base16_OceanLight,
}

impl SupportedCodeBlockTheme {
    pub fn to_theme(&self, ts: &ThemeSet) -> Theme {
        ts.themes[self.to_string().as_str()].clone()
    }
}

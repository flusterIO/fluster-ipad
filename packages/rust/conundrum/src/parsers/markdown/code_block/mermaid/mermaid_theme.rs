use serde::{Deserialize, Serialize};

#[typeshare::typeshare]
#[derive(Clone, Debug, Serialize, Default, Deserialize, strum_macros::Display)]
pub enum MermaidTheme {
    #[serde(rename = "auto")]
    #[strum(to_string = "auto")]
    #[default]
    Auto,
    #[serde(rename = "zinc-light")]
    #[strum(to_string = "zinc-light")]
    ZincLight,
    #[serde(rename = "zinc-dark")]
    #[strum(to_string = "zinc-dark")]
    ZincDark,
    #[serde(rename = "tokyo-night")]
    #[strum(to_string = "tokyo-night")]
    TokyoNight,
    #[serde(rename = "tokyo-night-storm")]
    #[strum(to_string = "tokyo-night-storm")]
    TokyoNightStorm,
    #[serde(rename = "tokyo-night-light")]
    #[strum(to_string = "tokyo-night-light")]
    TokyoNightLight,
    #[serde(rename = "catppuccin-mocha")]
    #[strum(to_string = "catppuccin-mocha")]
    CatppuccinMocha,
    #[serde(rename = "catppuccin-latte")]
    #[strum(to_string = "catppuccin-latte")]
    CatppuccinLatte,
    #[serde(rename = "nord")]
    #[strum(to_string = "nord")]
    Nord,
    #[serde(rename = "nord-light")]
    #[strum(to_string = "nord-light")]
    NordLight,
    #[serde(rename = "dracula")]
    #[strum(to_string = "dracula")]
    Dracula,
    #[serde(rename = "github-light")]
    #[strum(to_string = "github-light")]
    GithubLight,
    #[serde(rename = "github-dark")]
    #[strum(to_string = "github-dark")]
    GithubDark,
    #[serde(rename = "solarized-light")]
    #[strum(to_string = "solarized-light")]
    SolarizedLight,
    #[serde(rename = "solarized-dark")]
    #[strum(to_string = "solarized-dark")]
    SolarizedDark,
    #[serde(rename = "one-dark")]
    #[strum(to_string = "one-dark")]
    OneDark,
}

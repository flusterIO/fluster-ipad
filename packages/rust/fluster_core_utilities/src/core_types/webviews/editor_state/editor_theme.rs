use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(uniffi::Enum, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CodeEditorTheme {
    // Light
    MaterialLight,
    SolarizedLight,
    SolarizedDark,
    GithubLight,
    Aura,
    TokyoNightDay,
    XcodeLight,
    // Dark
    Dracula,
    TokyoNight,
    MaterialDark,
    TokyoNightStorm,
    GithubDark,
    XcodeDark,
}

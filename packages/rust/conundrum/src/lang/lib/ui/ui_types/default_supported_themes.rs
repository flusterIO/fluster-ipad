use serde::{Deserialize, Serialize};

/// ## Theme-ing
///
/// Themeing inside of [Fluster](https;//flusterapp.com) _was_ working, but
/// after migrating to our own independent compiler (the one you're looking at),
/// the theming has not yet been reimplemented. All of the structure is in
/// place, it's just a mater of finding a day that I can dedciate to theming
/// alone, but with the upcoming release of Fluster imininent I've decided to
/// focus on the release first.
#[derive(Serialize, Deserialize, Clone, strum_macros::Display, strum_macros::EnumIter)]
pub enum DefaultSupportedTheme {
    #[serde(rename = "fluster")]
    #[strum(to_string = "fluster")]
    Fluster,
    #[serde(rename = "zinc")]
    #[strum(to_string = "zinc")]
    Zinc,
    #[serde(rename = "yellow")]
    #[strum(to_string = "yellow")]
    Yellow,
    #[serde(rename = "violet")]
    #[strum(to_string = "violet")]
    Violet,
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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    lang::lib::ui::ui_types::emphasis::emphasis_model::Emphasis, parsers::conundrum::color::css_color::CssColor,
};

/// # EmphasisColors
///
/// These colors describe a set of styles the user can apply to specific
/// components to indicate a sort of 'mood'. For example, an `error` property to
/// indicate a failed result, or a `research` property to indicate a question
/// that needs further research.
///
/// As with all themes in Conundrum, all colors must be provided as a css
/// parsable string. Conundrum is using LightningCSS under the hood.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmphasisColors<ColorType = CssColor>(HashMap<Emphasis, ColorType>);

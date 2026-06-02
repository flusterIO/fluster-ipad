use serde::{Deserialize, Serialize};

use crate::parsers::conundrum::color::{color_pair::ColorPair, conundrum_color::ConundrumColor};

#[derive(Serialize, Deserialize, Clone)]
pub struct ColorSchemeGroup<T> {
    pub light: T,
    pub dark: T,
}

impl<T> ColorSchemeGroup<T> {
    pub fn new(light: T, dark: T) -> Self {
        Self { light,
               dark }
    }
}

pub type ConundrumCompleteColor = ColorSchemeGroup<ColorPair<ConundrumColor>>;

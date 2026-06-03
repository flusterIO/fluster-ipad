use serde::{Deserialize, Serialize};

use crate::{
    lang::lib::ui::ui_types::emphasis::emphasis_model::Emphasis,
    output::html::web_specific_traits::css_value_representable::CSSInlineHtmlValuePairRepresentable,
    parsers::conundrum::color::{color_pair::ColorPair, conundrum_color::ConundrumColor},
};

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

impl From<Emphasis> for ColorSchemeGroup<ColorPair<ConundrumColor>> {
    fn from(value: Emphasis) -> Self {
        let res = value.as_inline_style_value_group();
        ColorSchemeGroup { light: res.clone(),
                           dark: res.clone() }
    }
}

pub type ConundrumCompleteColor = ColorSchemeGroup<ColorPair<ConundrumColor>>;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::parsers::markdown::code_block::supported_themes::SupportedCodeBlockTheme;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, uniffi::Record, Clone)]
pub struct UIParams {
    pub dark_mode: bool,
    /// A number 0-n, where n > 1 increases the fontsize.
    pub font_scalar: f32,
    pub math_font_scalar: f32,
    pub syntax_theme: Option<SupportedCodeBlockTheme>,
}

impl Default for UIParams {
    fn default() -> Self {
        Self { dark_mode: true,
               font_scalar: 1.0,
               math_font_scalar: 1.2,
               syntax_theme: None }
    }
}

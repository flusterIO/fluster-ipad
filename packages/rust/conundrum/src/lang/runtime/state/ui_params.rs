use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, uniffi::Record, Clone)]
pub struct UIParams {
    pub dark_mode: bool,
    /// A number 0-n, where n > 1 increases the fontsize.
    pub font_scalar: u8,
}

impl Default for UIParams {
    fn default() -> Self {
        Self { dark_mode: true,
               font_scalar: 1 }
    }
}

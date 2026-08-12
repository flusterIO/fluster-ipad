use serde::{Deserialize, Serialize};

use crate::parsers::conundrum::color::css_color::CssColor;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChartColors<ColorType = CssColor> {
    /// A color used in charts for the first line, pie slice or bar mark.
    pub chart_1: ColorType,
    /// A color used in charts for the second line, pie slice or bar mark.
    pub chart_2: ColorType,
    /// A color used in charts for the third line, pie slice or bar mark.
    pub chart_3: ColorType,
    /// A color used in charts for the 4th line, pie slice or bar mark.
    pub chart_4: ColorType,
    /// A color used in charts for the 5th line, pie slice or bar mark.
    pub chart_5: ColorType,
    /// A color used in charts for the 6th line, pie slice or bar mark.
    pub chart_6: ColorType,
    /// A color used in charts for the 7th line, pie slice or bar mark.
    pub chart_7: ColorType,
    /// A color used in charts for the 8th line, pie slice or bar mark.
    pub chart_8: ColorType,
    /// A color used in charts for the 9th line, pie slice or bar mark.
    pub chart_9: ColorType,
    /// A color used in charts for the 10th line, pie slice or bar mark.
    pub chart_10: ColorType,
    /// A color used in charts for the 11th line, pie slice or bar mark.
    pub chart_11: ColorType,
    /// A color used in charts for the 12th line, pie slice or bar mark.
    pub chart_12: ColorType,
}

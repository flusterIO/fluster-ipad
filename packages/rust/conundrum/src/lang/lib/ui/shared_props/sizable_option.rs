use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::ui::ui_traits::jsx_prop_representable::{FromJsxPropsOptional, JsxPropRepresentable},
        runtime::state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult, ConundrumResult},
        },
    },
    parsers::conundrum::logic::object::object::ConundrumObject,
};

#[typeshare::typeshare]
#[derive(Serialize, uniffi::Enum, Deserialize, Debug, EnumIter, Display, Clone)]
pub enum SizableOption {
    #[serde(rename = "none")]
    #[strum(to_string = "none")]
    None,
    #[serde(rename = "small")]
    #[strum(to_string = "small")]
    Small,
    #[serde(rename = "smedium")]
    #[strum(to_string = "smedium")]
    Smedium,
    #[serde(rename = "medium")]
    #[strum(to_string = "medium")]
    Medium,
    #[serde(rename = "large")]
    #[strum(to_string = "large")]
    Large,
    #[serde(rename = "xl")]
    #[strum(to_string = "xl")]
    Xl,
    #[serde(rename = "xxl")]
    #[strum(to_string = "xxl")]
    Xxl,
    #[serde(rename = "fit")]
    #[strum(to_string = "fit")]
    Fit,
    #[serde(rename = "full")]
    #[strum(to_string = "full")]
    Full,
}

impl JsxPropRepresentable for SizableOption {
    fn to_jsx_prop(&self, key: &str) -> String {
        format!("{}=\"{}\"", key, self)
    }
}

impl FromStr for SizableOption {
    type Err = ConundrumErrorVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(SizableOption::None),
            "small" => Ok(SizableOption::Small),
            "smedium" => Ok(SizableOption::Smedium),
            "medium" => Ok(SizableOption::Medium),
            "large" => Ok(SizableOption::Large),
            "xl" => Ok(SizableOption::Xl),
            "xxl" => Ok(SizableOption::Xxl),
            "fit" => Ok(SizableOption::Fit),
            "full" => Ok(SizableOption::Full),
            _ => Err(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message(format!("Conundrum is looking for a 'SizableOption' and found another string: `{}`", s).as_str())))
        }
    }
}

impl FromJsxPropsOptional for SizableOption {
    /// Get's a sizable property applid as something like width="small".
    fn from_jsx_props(props: &ConundrumObject, key: &str) -> ConundrumModalResult<Self>
        where Self: Sized {
        let k_value =
            props.get_string(key,
                        Some("Failed to get a valid sizable property. Please see the `Sizable` docs for more information.")).map_err(ErrMode::Backtrack)?;
        SizableOption::from_str(k_value.0.as_str()).map_err(|e| ErrMode::Backtrack(e))
    }
}

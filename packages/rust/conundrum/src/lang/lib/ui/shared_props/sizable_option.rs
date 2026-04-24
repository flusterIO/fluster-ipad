use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};
use winnow::error::ErrMode;

use crate::{
    lang::{
        constants::ui_constants::MAX_DOCUMENT_WIDTH,
        elements::parsed_elements::ParsedElement,
        lib::ui::ui_traits::jsx_prop_representable::{FromJsxPropsOptional, JsxPropRepresentable},
        runtime::state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        },
    },
    parsers::conundrum::logic::object::object::ConundrumObject,
};

#[typeshare::typeshare]
#[derive(Serialize, uniffi::Enum, Deserialize, Debug, EnumIter, Display, Clone, PartialEq, Eq, Hash)]
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

impl SizableOption {
    // A simple wrapper because I can't figure out how to import the iter function
    // and I don't want to waste time on it...
    pub fn all_sizable_options() -> Vec<SizableOption> {
        SizableOption::iter().collect::<Vec<SizableOption>>()
    }

    pub fn one_smaller(&self) -> Option<Self> {
        match self {
            Self::None => None,
            Self::Small => Some(Self::None),
            Self::Smedium => Some(Self::Small),
            Self::Medium => Some(Self::Smedium),
            Self::Large => Some(Self::Medium),
            Self::Xl => Some(Self::Large),
            Self::Xxl => Some(Self::Xl),
            Self::Full => Some(Self::Xxl),
            Self::Fit => Some(Self::Full),
        }
    }

    /// Returns an `Option<SizableOption` from a `{[K in SizableOption]?:
    /// boolean}`
    pub fn from_jsx_props_bool_record(props: &ConundrumObject) -> Option<Self> {
        for item in SizableOption::iter() {
            if let Some(data) = props.data.get(&item.to_string()) {
                if let Some(res) = match data.value() {
                    ParsedElement::Logic(l) => match l {
                        crate::parsers::conundrum::logic::token::ConundrumLogicToken::Bool(b) => {
                            if b.0 {
                                Some(item)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    },
                    _ => None,
                } {
                    return Some(res);
                }
            }
        }
        None
    }

    pub fn to_css_column_class(&self, n_columns: Option<i32>) -> String {
        let n = n_columns.unwrap_or_else(|| self.to_default_grid_cols());
        // This can't be scanned by tailwind or shit will absolutely explode.
        format!("@[{}px]/mdx:grid-cols-{}", self.to_column_breakpoint(), n)
    }

    /// The default number of columns at the default breakpoint for this size.
    pub fn to_default_grid_cols(&self) -> i32 {
        match self {
            Self::None => 1,
            Self::Small => 1,
            Self::Smedium => 1,
            Self::Medium => 1,
            Self::Large => 2,
            Self::Xl => 2,
            Self::Xxl => 2,
            Self::Full => 2,
            Self::Fit => 4,
        }
    }

    /// The breakpoint at which this size should collapse to a single column.
    pub fn to_column_breakpoint(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Small => 120,
            Self::Smedium => 180,
            Self::Medium => 240,
            Self::Large => 320,
            Self::Xl => 480,
            Self::Xxl => 640,
            Self::Full => 800,
            Self::Fit => 1200,
        }
    }
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

impl SizableOption {
    /// The point at which a sizable component expands to be full width,
    /// depending on the size obviously.
    pub fn to_width_css_breakpoint(&self) -> u16 {
        match self {
            SizableOption::None => 0,
            SizableOption::Small => 320,
            SizableOption::Smedium => 480,
            SizableOption::Medium => 640,
            SizableOption::Large => 768,
            SizableOption::Xl => 896,
            SizableOption::Xxl => 1024,
            SizableOption::Fit => 0,
            SizableOption::Full => MAX_DOCUMENT_WIDTH.clone() as u16,
        }
    }
}

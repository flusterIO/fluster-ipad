use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use typeshare::typeshare;
use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        runtime::state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult, ConundrumResult},
        },
    },
    output::output_components::text_wrappers::inline_markdown_override_text::{
        bold_italic_text, bold_text, italic_text,
    },
    parsers::{
        conundrum::logic::object::object::ConundrumObject,
        javascript::parsed_javascript_elements::ParsedJavascriptElement,
    },
};

/// When a component is rendered in a markdown environment, apply these styles
/// if the more complex rendering strategies are not available.
#[typeshare]
#[derive(Serialize, uniffi::Enum, Debug, Deserialize, Display, EnumIter, PartialEq, Eq, Clone, Copy)]
pub enum InlineMarkdownOverride {
    #[serde(rename = "plain")]
    #[strum(to_string = "plain")]
    Plain,
    #[serde(rename = "bold_italic")]
    #[strum(to_string = "bold_italic")]
    BoldItalic,
    #[serde(rename = "italic")]
    #[strum(to_string = "italic")]
    Italic,
    #[serde(rename = "bold")]
    #[strum(to_string = "bold")]
    Bold,
}

impl FromStr for InlineMarkdownOverride {
    type Err = ConundrumErrorVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(InlineMarkdownOverride::Plain),
            "bold_italic" => Ok(InlineMarkdownOverride::BoldItalic),
            "boldItalic" => Ok(InlineMarkdownOverride::BoldItalic),
            "italic" => Ok(InlineMarkdownOverride::Italic),
            "bold" => Ok(InlineMarkdownOverride::Bold),
            _ => {
                Err(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Failed to find markdown format override.", format!("It looks like you put in `{}` for the `markdown` field but this field is one of the `InlineMarkdownOverride` variants.", s).as_str())))
            }
        }
    }
}

impl InlineMarkdownOverride {
    pub fn wrap_content(&self, content: &str) -> String {
        match self {
            InlineMarkdownOverride::Italic => italic_text(content),
            InlineMarkdownOverride::Bold => bold_text(content),
            InlineMarkdownOverride::BoldItalic => bold_italic_text(content),
            InlineMarkdownOverride::Plain => content.to_string(),
        }
    }
}

impl FromJsxPropsOptional for InlineMarkdownOverride {
    fn from_jsx_props(props: &ConundrumObject, key: &str) -> ConundrumModalResult<Self>
        where Self: Sized {
        props.data.get(key).map(|l| {
            match l.value() {
                ParsedElement::Javascript(js) => match js {
                    ParsedJavascriptElement::String(s) => Some(s.value.clone()),
                    _ => None
                },
                _ => None
            }.map(|user_string| {

        InlineMarkdownOverride::from_str(user_string.as_str()).map_err(|e| {
            ErrMode::Backtrack(ConundrumErrorVariant::UserFacingMissingOrIncorrectProperty(ConundrumError::from_message("Conundrum received a value for the markdown override format that is not supported.")))
        })
            })
        })
        .ok_or(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Fail to find markdown override property."))))
            ?
        .ok_or(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Fail to find markdown override property."))))
        ?
    }
}

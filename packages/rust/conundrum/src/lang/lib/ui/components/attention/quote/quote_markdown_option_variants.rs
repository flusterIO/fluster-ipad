use std::str::FromStr;

use askama::Template;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use strum::{EnumString, IntoEnumIterator};
use strum_macros::{Display, EnumIter};
use typeshare::typeshare;
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::ui::{
            shared_props::markdown_component_options::markdown_component_options_model::{
                MarkdownComponentOptionData, MarkdownComponentOptionMap, MarkdownComponentOptions,
                MarkdownOutputVariantKey,
            },
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        },
        runtime::state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        },
    },
    parsers::conundrum::logic::{object::object::ConundrumObject, string::conundrum_string::ConundrumString},
};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Display, EnumIter, EnumString, uniffi::Enum, Clone)]
pub enum QuoteMarkdownVariants {
    #[serde(rename = "hide")]
    #[strum(to_string = "hide")]
    Hide,
    #[serde(rename = "blockquote")]
    #[strum(to_string = "blockquote")]
    BlockQuote,
    #[serde(rename = "blockquote+source")]
    #[strum(to_string = "blockquote+source")]
    BlockQuoteAndSource,
}

impl FromJsxPropsOptional for MarkdownComponentOptions<QuoteMarkdownVariants> {
    fn from_jsx_props(props: &ConundrumObject, key: &str) -> ConundrumModalResult<Self>
        where Self: Sized {
        if let Ok(s) = ConundrumString::from_jsx_props(&props, key) {
            QuoteMarkdownVariants::from_str(s.0.as_str())
                .map(|c| {
                    MarkdownComponentOptions {
                        markdown: MarkdownComponentOptionData::AsVariant(c)
                    }
                })
                .map_err(|_| {
                let mut l = String::from("");
                for item in QuoteMarkdownVariants::iter() {
                     l += item.to_string().as_str();
                };
                ErrMode::Cut(
ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid key", format!(r#"The markdown property you tried to set for the `{}` is invalid. Please provide one of the following: 

{}"#, key, l).as_str()))
                )
            })
        } else if let Ok(o) = ConundrumObject::from_jsx_props(&props, key) {
            let data: DashMap<MarkdownOutputVariantKey, Option<QuoteMarkdownVariants>> = DashMap::new();
            let mut has_data = false;
            for item in MarkdownOutputVariantKey::iter() {
                if let Some(s) = o.get_string(item.to_string().as_str(), None).ok() {
                    if let Some(res) = QuoteMarkdownVariants::from_str(s.0.as_str()).ok() {
                        has_data = true;
                        data.insert(item, Some(res));
                    }
                }
            }
            if has_data {
                Ok(MarkdownComponentOptions { markdown:
                                                  MarkdownComponentOptionData::AsMap(MarkdownComponentOptionMap(data)) })
            } else {
                Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Incomplete or invalid map"))))
            }
        } else {
            Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("No data found"))))
        }
    }
}

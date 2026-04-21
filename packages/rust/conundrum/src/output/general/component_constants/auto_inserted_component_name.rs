use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use typeshare::typeshare;

use crate::lang::runtime::state::{conundrum_error::ConundrumError, conundrum_error_variant::ConundrumErrorVariant};

#[typeshare]
#[derive(Display, Debug, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize, Clone, uniffi::Enum)]
pub enum AutoInsertedComponentName {
    #[serde(rename = "NoteLink")]
    #[strum(to_string = "NoteLink")]
    NoteLink,
    #[serde(rename = "AutoInsertedTag")]
    #[strum(to_string = "AutoInsertedTag")]
    AutoInsertedTag,
    #[serde(rename = "FlusterCitation")]
    #[strum(to_string = "FlusterCitation")]
    FlusterCitation,
    #[serde(rename = "DictionaryEntry")]
    #[strum(to_string = "DictionaryEntry")]
    DictionaryEntry,
    #[serde(rename = "FlusterAiParsePendingContainer")]
    #[strum(to_string = "FlusterAiParsePendingContainer")]
    FlusterAiParsePendingContainer,
    // Markdown
    #[serde(rename = "AutoInsertedHeading")]
    #[strum(to_string = "AutoInsertedHeading")]
    AutoInsertedHeading,
    #[serde(rename = "AutoInsertedCodeBlock")]
    #[strum(to_string = "AutoInsertedCodeBlock")]
    AutoInsertedCodeBlock,
    #[serde(rename = "AutoInsertedBlockQuote")]
    #[strum(to_string = "AutoInsertedBlockQuote")]
    AutoInsertedBlockQuote,
    #[serde(rename = "AutoInsertedMathBlock")]
    #[strum(to_string = "AutoInsertedMathBlock")]
    AutoInsertedMathBlock,
    #[serde(rename = "AutoInsertedMarkdownLink")]
    #[strum(to_string = "AutoInsertedMarkdownLink")]
    AutoInsertedMarkdownLink,
    #[serde(rename = "AutoInsertedMarkdownParagraph")]
    #[strum(to_string = "AutoInsertedMarkdownParagraph")]
    AutoInsertedMarkdownParagraph,
}

impl FromStr for AutoInsertedComponentName {
    type Err = ConundrumErrorVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for item in AutoInsertedComponentName::iter() {
            if item.to_string() == s.to_string() {
                return Ok(item);
            }
        }
        Err(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Fail to find auto inserted component. This aint good.")))
    }
}

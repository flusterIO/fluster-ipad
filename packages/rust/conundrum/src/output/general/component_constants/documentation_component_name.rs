use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use typeshare::typeshare;

use crate::lang::runtime::state::{conundrum_error::ConundrumError, conundrum_error_variant::ConundrumErrorVariant};

#[typeshare]
#[derive(Display, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentationComponentName {
    #[serde(rename = "InContentDocumentationContainer")]
    #[strum(to_string = "InContentDocumentationContainer")]
    InContentDocumentationContainer,
    #[serde(rename = "InContentDocsEmphasisTypeList")]
    #[strum(to_string = "InContentDocsEmphasisTypeList")]
    InContentDocsEmphasisTypeList,
    #[serde(rename = "InContentDocsHighlightDemo")]
    #[strum(to_string = "InContentDocsHighlightDemo")]
    InContentDocsHighlightDemo,
    #[serde(rename = "InContentDocsUnderlineDemo")]
    #[strum(to_string = "InContentDocsUnderlineDemo")]
    InContentDocsUnderlineDemo,
    #[serde(rename = "AutoInsertedNestedEmojiDocumentation")]
    #[strum(to_string = "AutoInsertedNestedEmojiDocumentation")]
    EmojiDocumentationDemo,
}

impl FromStr for DocumentationComponentName {
    type Err = ConundrumErrorVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for item in DocumentationComponentName::iter() {
            if item.to_string() == s.to_string() {
                return Ok(item);
            }
        }
        Err(ConundrumErrorVariant::InternalParserError(ConundrumError::from_message("Fail to find documentation component.")))
    }
}

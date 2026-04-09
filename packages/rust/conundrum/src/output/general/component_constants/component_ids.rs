use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};
use typeshare::typeshare;

use crate::{
    embedded::in_content_documentation_id::InContentDocumentationFormat,
    lang::runtime::state::{conundrum_error::ConundrumError, conundrum_error_variant::ConundrumErrorVariant},
};

/// From typescript to swift.
#[typeshare]
#[derive(strum_macros::Display, PartialEq, Eq, Hash, Clone, Copy, EnumIter, Serialize, Deserialize, uniffi::Enum)]
pub enum EmbeddableComponentId {
    #[serde(rename = "admonition")]
    #[strum(to_string = "admonition")]
    Admonition,
    #[serde(rename = "highlight")]
    #[strum(to_string = "highlight")]
    Hl,
    #[serde(rename = "underline")]
    #[strum(to_string = "underline")]
    Ul,
    #[serde(rename = "card")]
    #[strum(to_string = "card")]
    Card,
    #[serde(rename = "grid")]
    #[strum(to_string = "grid")]
    Grid,
    #[serde(rename = "util-container")]
    #[strum(to_string = "util-container")]
    UtlityContainer,
    #[serde(rename = "hr-with-children")]
    #[strum(to_string = "hr-with-children")]
    HrWithChildren,
    #[serde(rename = "embeddable-hint-component")]
    #[strum(to_string = "embeddable-hint-component")]
    Hint,
    #[serde(rename = "emoji-component")]
    #[strum(to_string = "emoji-component")]
    Emoji,
    #[serde(rename = "tab-group")]
    #[strum(to_string = "tab-group")]
    Tabs,
    #[serde(rename = "tab-group-tab")]
    #[strum(to_string = "tab-group-tab")]
    Tab,
    #[serde(rename = "equation-reference")]
    #[strum(to_string = "equation-reference")]
    EqRef,
    #[serde(rename = "ai-note-summary")]
    #[strum(to_string = "ai-note-summary")]
    AINoteSummary,
    // #[serde(rename = "ai-research-suggestions")]
    // #[strum(to_string = "ai-research-suggestions")]
    // AIResearchSuggestions,
    // #[serde(rename = "ai-generated-image")]
    // #[strum(to_string = "ai-generated-image")]
    // AIGeneratedImage,
}

impl FromStr for EmbeddableComponentId {
    type Err = ConundrumErrorVariant;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        EmbeddableComponentId::iter().find(|f| {
            f.to_string() == s
        }).ok_or(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Conundrum tried to convert an invalid string to a component name.", "This likely means that you tried to create a component that does not exist.")))
    }
}

impl EmbeddableComponentId {
    // Must match the file in the embedded directory.
    pub fn to_embedded_file_name(self, format: &InContentDocumentationFormat) -> String {
        format!("{}-{}.mdx", self, format)
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::{
        embedded::in_content_documentation_id::get_file_names_in_dir,
        testing::get_workspace_root::{self, get_workspace_root},
    };

    use super::*;

    #[test]
    fn all_in_content_documentation_exists() {
        let root = get_workspace_root();
        let p = std::path::Path::new(&root).join("packages")
                                           .join("rust")
                                           .join("conundrum")
                                           .join("src")
                                           .join("embedded")
                                           .join("component_docs");
        let file_names =
            get_file_names_in_dir(&p).expect("Reads 'component_docs' notes directory without throwing an error.");
        for doc_format in InContentDocumentationFormat::iter() {
            for id in EmbeddableComponentId::iter() {
                let file_name_should_exist = id.to_embedded_file_name(&doc_format.clone());
                assert!(file_names.iter().any(|x| x == &file_name_should_exist),
                        "The {} does not appear to exist.",
                        file_name_should_exist)
            }
        }

        // assert_eq!(result, 4);
    }
}

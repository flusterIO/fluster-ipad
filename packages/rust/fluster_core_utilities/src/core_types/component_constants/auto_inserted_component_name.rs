use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use typeshare::typeshare;

#[typeshare]
#[derive(Display, EnumIter, Hash, PartialEq, Eq, Serialize, Deserialize)]
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
    #[serde(rename = "AutoInsertedBlockQuote")]
    #[strum(to_string = "AutoInsertedBlockQuote")]
    AutoInsertedBlockQuote,
    #[serde(rename = "AutoInsertedBlockMath")]
    #[strum(to_string = "AutoInsertedBlockMath")]
    AutoInsertedBlockMath,
    #[serde(rename = "AutoInsertedInlineMath")]
    #[strum(to_string = "AutoInsertedInlineMath")]
    AutoInsertedInlineMath,
    #[serde(rename = "AutoInsertedMarkdownLink")]
    #[strum(to_string = "AutoInsertedMarkdownLink")]
    AutoInsertedMarkdownLink,
    #[serde(rename = "AutoInsertedMarkdownParagraph")]
    #[strum(to_string = "AutoInsertedMarkdownParagraph")]
    AutoInsertedMarkdownParagraph,
}

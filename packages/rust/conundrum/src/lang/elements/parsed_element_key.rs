use std::fmt::Display;

use serde::{Deserialize, Serialize};
#[typeshare::typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, uniffi::Enum, strum_macros::Display)]
#[serde(tag = "tag", content = "content")]
pub enum ParsedElementKey {
    // Markdown
    #[serde(rename = "heading")]
    #[strum(to_string = "heading")]
    Heading,
    #[serde(rename = "block-quote")]
    #[strum(to_string = "block-quote")]
    BlockQuote,
    #[serde(rename = "block-math")]
    #[strum(to_string = "block-math")]
    BlockMath,
    #[serde(rename = "inline-math")]
    #[strum(to_string = "inline-math")]
    InlineMath,
    #[serde(rename = "text")]
    #[strum(to_string = "text")]
    Text,
    #[serde(rename = "escaped-str")]
    #[strum(to_string = "escaped-str")]
    EscapedStr,
    #[serde(rename = "hr")]
    #[strum(to_string = "hr")]
    Hr,
    #[serde(rename = "bold-text")]
    #[strum(to_string = "bold-text")]
    BoldText,
    #[serde(rename = "italic-text")]
    #[strum(to_string = "italic-text")]
    ItalicText,
    #[serde(rename = "bold-italic-text")]
    #[strum(to_string = "bold-italic-text")]
    BoldAndItalicText,
    #[serde(rename = "strike-through-text")]
    #[strum(to_string = "boldanditalictext")]
    StrikethroughText,
    #[serde(rename = "code-block")]
    #[strum(to_string = "code-block")]
    ParsedCodeBlock,
    #[serde(rename = "inline-code-block")]
    #[strum(to_string = "inline-code-block")]
    InlineCode,
    #[serde(rename = "md-link")]
    #[strum(to_string = "md-link")]
    MarkdownLink,
    #[serde(rename = "paragraph")]
    #[strum(to_string = "paragraph")]
    MarkdownParagraph,
    #[serde(rename = "unordered-list")]
    #[strum(to_string = "unordered-list")]
    UnorderedList,
    #[serde(rename = "ordered-list")]
    #[strum(to_string = "ordered-list")]
    OrderedList,
    #[serde(rename = "table")]
    #[strum(to_string = "table")]
    Table,
    #[serde(rename = "footnote-anchor")]
    #[strum(to_string = "footnote-anchor")]
    FootnoteAnchor,
    #[serde(rename = "footnote-footer")]
    #[strum(to_string = "footnote-footer")]
    FootnoteFooter,
    #[serde(rename = "emoji")]
    #[strum(to_string = "emoji")]
    Emoji,
    #[serde(rename = "comp-self-closing")]
    #[strum(to_string = "comp-self-closing")]
    ReactComponentSelfClosing,
    #[serde(rename = "comp-with-children")]
    #[strum(to_string = "comp-with-children")]
    ReactComponentWithChildren,
    #[serde(rename = "citation")]
    #[strum(to_string = "citation")]
    ParsedCitation,
    #[serde(rename = "outgoing-note-link")]
    #[strum(to_string = "outgoing-note-link")]
    ParsedOutgoingNoteLink,
    #[serde(rename = "tag")]
    #[strum(to_string = "tag")]
    Tag,
    #[serde(rename = "inspection-request")]
    #[strum(to_string = "inspection-request")]
    ParsedInspectionRequest,
    #[serde(rename = "hr-with-children")]
    #[strum(to_string = "hr-with-children")]
    HrWithChildren,
    #[serde(rename = "comment")]
    #[strum(to_string = "comment")]
    Comment,
    #[serde(rename = "children")]
    #[strum(to_string = "children")]
    Children,
    #[serde(rename = "javascript")]
    #[strum(to_string = "javascript")]
    Javascript,
    #[serde(rename = "logic")]
    #[strum(to_string = "logic")]
    Logic,
}

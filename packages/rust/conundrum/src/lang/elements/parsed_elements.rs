use serde::Serialize;

use crate::{
    lang::runtime::{
        state::parse_state::ParseState,
        traits::{fluster_component_result::FlusterComponentResult, mdx_component_result::MdxComponentResult},
    },
    parsers::{
        fluster::{
            docs::ParsedInspectionRequest, fluster_comment::FlusterCommentResult,
            hr_with_children::HrWithChildrenResult, inline_citation::ParsedCitation, note_link::ParsedOutgoingNoteLink,
            tag::ParsedTag,
        },
        markdown::{
            block_math::BlockMathResult, block_quote::BlockQuoteResult,
            bold_and_italic_text::MarkdownBoldAndItalicTextResult, bold_text::MarkdownBoldTextResult,
            code_block::ParsedCodeBlock, heading::MarkdownHeadingResult, inline_math::InlineMathResult,
            italic_text::MarkdownItalicTextResult, markdown_link::MarkdownLinkResult,
            paragraph::MarkdownParagraphResult,
        },
    },
};

impl MdxComponentResult for String {
    fn to_mdx_component(&self, _: &mut ParseState) -> String {
        self.clone()
    }
}

// This enum acts as a router for our different syntaxes
// As a current limitation, a parser cannot have children as
// `Vec<ParsedElement>` or reference `ParsedElement` in any other way _and_
// export it's type via typeshare.
#[derive(Debug, Serialize)]
pub enum ParsedElement {
    // Markdown
    Heading(MarkdownHeadingResult),
    BlockQuote(BlockQuoteResult),
    BlockMath(BlockMathResult),
    InlineMath(InlineMathResult),
    Text(String),
    BoldText(MarkdownBoldTextResult),
    ItalicText(MarkdownItalicTextResult),
    BoldAndItalicText(MarkdownBoldAndItalicTextResult),
    ParsedCodeBlock(ParsedCodeBlock),
    MarkdownLink(MarkdownLinkResult),
    MarkdownParagraph(MarkdownParagraphResult),
    // Fluster Specific
    ParsedCitation(ParsedCitation),
    ParsedOutgoingNoteLink(ParsedOutgoingNoteLink),
    Tag(ParsedTag),
    ParsedInspectionRequest(ParsedInspectionRequest),
    HrWithChildren(HrWithChildrenResult),
    Comment(FlusterCommentResult),
}

impl MdxComponentResult for ParsedElement {
    fn to_mdx_component(&self, res: &mut ParseState) -> String {
        match self {
            ParsedElement::ParsedInspectionRequest(req) => req.to_mdx_component(res),
            ParsedElement::ParsedCodeBlock(block) => block.to_mdx_component(res),
            ParsedElement::ParsedCitation(cite) => cite.to_mdx_component(res),
            ParsedElement::ParsedOutgoingNoteLink(l) => l.to_mdx_component(res),
            ParsedElement::Tag(tag) => tag.to_mdx_component(res),
            ParsedElement::Text(s) => s.clone(),
            ParsedElement::Heading(heading) => heading.to_mdx_component(res),
            ParsedElement::BlockMath(math) => math.to_mdx_component(res),
            ParsedElement::InlineMath(math) => math.to_mdx_component(res),
            ParsedElement::BlockQuote(quote) => quote.to_mdx_component(res),
            ParsedElement::BoldText(t) => t.to_fluster_component(res),
            ParsedElement::ItalicText(t) => t.to_fluster_component(res),
            ParsedElement::BoldAndItalicText(t) => t.to_fluster_component(res),
            ParsedElement::MarkdownLink(l) => l.to_mdx_component(res),
            ParsedElement::MarkdownParagraph(p) => p.to_mdx_component(res),
            ParsedElement::HrWithChildren(c) => c.to_mdx_component(res),
            ParsedElement::Comment(c) => c.to_mdx_component(res),
        }
    }
}

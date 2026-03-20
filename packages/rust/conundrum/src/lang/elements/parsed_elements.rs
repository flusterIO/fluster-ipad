use crate::{
    lang::runtime::traits::mdx_component_result::MdxComponentResult,
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
    parsers::{
        fluster::{
            docs::ParsedInspectionRequest, inline_citation::ParsedCitation, note_link::ParsedOutgoingNoteLink,
            tag::ParsedTag,
        },
        markdown::{
            block_math::BlockMathResult, block_quote::BlockQuoteResult, code_block::ParsedCodeBlock,
            heading::MarkdownHeadingResult, inline_math::InlineMathResult,
        },
    },
};

impl MdxComponentResult for String {
    fn to_mdx_component(&self, _: &mut MdxParsingResult) -> String {
        self.clone()
    }
}

// This enum acts as a router for our different syntaxes
#[derive(Debug)]
pub enum ParsedElement {
    // Markdown
    Heading(MarkdownHeadingResult),
    BlockQuote(BlockQuoteResult),
    BlockMath(BlockMathResult),
    InlineMath(InlineMathResult),
    Text(String),
    NestedContent(String),
    ParsedCodeBlock(ParsedCodeBlock),
    // Fluster Specific
    ParsedCitation(ParsedCitation),
    ParsedOutgoingNoteLink(ParsedOutgoingNoteLink),
    Tag(ParsedTag),
    ParsedInspectionRequest(ParsedInspectionRequest),
}

impl MdxComponentResult for ParsedElement {
    fn to_mdx_component(&self, res: &mut MdxParsingResult) -> String {
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
            ParsedElement::NestedContent(s) => s.clone(),
        }
    }
}

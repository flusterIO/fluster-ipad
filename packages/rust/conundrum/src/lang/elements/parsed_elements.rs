use crate::{
    lang::{elements::parsed_code_block::ParsedCodeBlock, runtime::traits::mdx_component_result::MdxComponentResult},
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
    parsers::fluster::{
        docs::ParsedInspectionRequest, inline_citation::ParsedCitation, note_link::ParsedOutgoingNoteLink,
        tag::ParsedTag,
    },
};

impl MdxComponentResult for String {
    fn to_mdx_component(&self, _: &mut MdxParsingResult) -> String {
        self.clone()
    }
}

// This enum acts as a router for our different syntaxes
pub enum ParsedElement {
    ParsedCodeBlock(ParsedCodeBlock),
    ParsedCitation(ParsedCitation),
    ParsedOutgoingNoteLink(ParsedOutgoingNoteLink),
    Tag(ParsedTag),
    ParsedInspectionRequest(ParsedInspectionRequest),
    Text(String),
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
        }
    }
}

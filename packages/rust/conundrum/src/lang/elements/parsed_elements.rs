use crate::{
    lang::{
        elements::parsed_code_block::ParsedCodeBlock,
        runtime::traits::mdx_component_result::MdxComponentResult,
    },
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
    parsers::fluster::{docs::ParsedInspectionRequest, inline_citation::ParsedCitation},
};

// This enum acts as a router for our different syntaxes
pub enum ParsedElement {
    CodeBlock(ParsedCodeBlock),
    Citation(ParsedCitation),
    InspectionRequest(ParsedInspectionRequest),
}

impl MdxComponentResult for ParsedElement {
    fn to_mdx_component(&self, res: &mut MdxParsingResult) -> String {
        match self {
            ParsedElement::CodeBlock(block) => block.to_mdx_component(res),
            ParsedElement::Citation(cite) => cite.to_mdx_component(res),
            ParsedElement::InspectionRequest(req) => req.to_mdx_component(res),
        }
    }
}

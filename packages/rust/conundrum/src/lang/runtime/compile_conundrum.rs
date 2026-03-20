use crate::{
    output::{
        output_components::{
            ai_parsing_request_phase_1::get_ai_parsing_request_phase_1_content::get_ai_parsing_request_phase_1_content,
            dictionary_entry::get_dictionary_entry_content::get_dictionary_content,
        },
        parsing_result::mdx_parsing_result::MdxParsingResult,
    },
    parsers::markdown::code_block::ParsedCodeBlock,
};

// Add this helper to compile specific languages into MDX components
// Moved this to the ParsedCodeBlock trait.
pub fn code_block_to_mdx(block: &ParsedCodeBlock, res: &mut MdxParsingResult) -> String {
    match block.language.as_str() {
        "dictionary" => {
            // Extract the metadata or provide a fallback
            get_dictionary_content(block, res)
        }
        "fluster-ai" => get_ai_parsing_request_phase_1_content(block, res),
        _ => {
            // For standard code blocks (like tsx, rust, etc.), leave them exactly as they
            // are and let mdx handle it for now.
            block.full_match.clone()
        }
    }
}

use crate::lang::elements::parsed_elements::ParsedElement;
use crate::lang::runtime::traits::mdx_component_result::MdxComponentResult;

/// Renders a slice of parsed elements into a single MDX string.
/// This is the shared rendering primitive used by both the top-level
/// `run_conundrum` entry point and any container element (block quote, etc.)
/// that needs to recursively render its inner content.
pub fn compile_elements(elements: &[ParsedElement], res: &mut MdxParsingResult) -> String {
    elements.iter().map(|e| e.to_mdx_component(res)).collect()
}

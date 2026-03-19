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

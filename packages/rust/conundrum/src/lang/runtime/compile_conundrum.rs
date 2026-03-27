use crate::lang::runtime::state::parse_state::ParseState;

use crate::lang::elements::parsed_elements::ParsedElement;
use crate::lang::runtime::traits::mdx_component_result::MdxComponentResult;

/// Renders a slice of parsed elements into a single MDX string.
/// This is the shared rendering primitive used by both the top-level
/// `run_conundrum` entry point and any container element (block quote, etc.)
/// that needs to recursively render its inner content.
pub fn compile_elements(elements: &[ParsedElement], res: &mut ParseState) -> String {
    elements.iter().map(|e| e.to_mdx_component(res)).collect()
}

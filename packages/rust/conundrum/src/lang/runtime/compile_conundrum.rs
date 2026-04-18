use core::panic;

use crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult;
use crate::lang::runtime::state::parse_state::{ConundrumCompileTarget, ParseState};

use crate::lang::elements::parsed_elements::ParsedElement;
use crate::lang::runtime::traits::html_js_component_result::HtmlJsComponentResult;
use crate::lang::runtime::traits::mdx_component_result::MdxComponentResult;

/// Renders a slice of parsed elements into a single MDX string.
/// This is the shared rendering primitive used by both the top-level
/// `run_conundrum` entry point and any container element (block quote, etc.)
/// that needs to recursively render its inner content.
pub fn compile_elements(elements: &[ParsedElement], res: &mut ParseState) -> ConundrumModalResult<String> {
    let mut s = String::from("");
    for em in elements.iter() {
        let component_res = match res.compile_target {
            ConundrumCompileTarget::Html => em.to_html_js_component(res),
            _ => {
                panic!("You're early. Right now Conundrum is being built with the HTML+JS target in mind for performance and portability, but a jsx target will be available shortly.")
            }
        }?;
        s += component_res.as_str();
    }
    Ok(s)
}

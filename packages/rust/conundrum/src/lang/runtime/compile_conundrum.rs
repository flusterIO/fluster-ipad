use crate::lang::elements::parsed_elements::ParsedElement;
use crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult;
use crate::lang::runtime::state::parse_state::ConundrumCompileTarget;
use crate::lang::runtime::traits::conundrum_input::ArcState;
use crate::lang::runtime::traits::html_js_component_result::HtmlJsComponentResult;
use crate::lang::runtime::traits::markdown_component_result::MarkdownComponentResult;
use crate::lang::runtime::traits::plain_text_component_result::PlainTextComponentResult;
use core::panic;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::sync::Arc;

/// Renders a slice of parsed elements into a single MDX string.
/// This is the shared rendering primitive used by both the top-level
/// `run_conundrum` entry point and any container element (block quote, etc.)
/// that needs to recursively render its inner content.
pub fn compile_elements(elements: &[ParsedElement], res: &ArcState) -> ConundrumModalResult<String> {
    let mut s = String::from("");
    for em in elements.iter() {
        let state = res.read_arc();
        let component_res = match state.compile_target {
            ConundrumCompileTarget::Html => {
                drop(state);
                em.to_html_js_component(Arc::clone(&res))
            }
            ConundrumCompileTarget::PlainText => {
                drop(state);
                em.to_plain_text(Arc::clone(res))
            }
            ConundrumCompileTarget::Markdown => {
                drop(state);
                em.to_markdown(Arc::clone(res))
            }
            _ => {
                panic!("You're early. Right now Conundrum is being built with the HTML+JS target in mind for performance and portability, but a jsx target will be available shortly.")
            }
        }?;
        s += component_res.as_str();
    }
    Ok(s)
}

pub fn compile_elements_multi_threaded(elements: &[ParsedElement], res: &ArcState) -> ConundrumModalResult<String> {
    let r: String = elements.par_iter().filter_map(|em| {
        let state = res.read_arc();
        match state.compile_target {
            ConundrumCompileTarget::Html => {
                drop(state);
                em.to_html_js_component(Arc::clone(res))
            },
            ConundrumCompileTarget::PlainText => {
                drop(state);
                em.to_plain_text(Arc::clone(res))
            },
            ConundrumCompileTarget::Markdown => {
                drop(state);
                em.to_markdown(Arc::clone(res))
            }
            _ => {
                panic!("You're early. Right now Conundrum is being built with the HTML+JS target in mind for performance and portability, but a jsx target will be available shortly.")
            }
        }.ok()
    }).collect::<Vec<String>>().join("");
    Ok(r)
}

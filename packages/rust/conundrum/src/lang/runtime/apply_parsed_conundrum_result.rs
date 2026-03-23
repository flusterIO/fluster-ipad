use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};

/// Applies the parsed conundrum stateful input to the result type.
/// This should be called only once at the end of the execution.
pub fn apply_parsed_conundrum_input_state(input: &ConundrumInput) -> MdxParsingResult {
    let mut state = input.state.borrow_mut();
    state.data.ordered_citation_keys = state.bib.data.clone();
    state.data.clone()
}

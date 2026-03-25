use crate::lang::runtime::{state::parse_state::ParseState, traits::conundrum_input::ConundrumInput};

/// Applies the parsed conundrum stateful input to the result type.
/// This should be called only once at the end of the execution.
pub fn apply_parsed_conundrum_input_state(input: &ConundrumInput) -> ParseState {
    let mut state = input.state.borrow_mut();
    state.data.ordered_citation_keys = state.bib.data.clone();
    state.clone()
}

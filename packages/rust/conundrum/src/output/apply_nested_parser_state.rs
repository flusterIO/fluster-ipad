use crate::lang::runtime::traits::conundrum_input::ConundrumInput;

/// Applies the nested children parser state to the parent state after the
/// children have been parsed.
/// This will need to be improved **significantly**. It's basically a
/// placeholder now for more complicated state moving forward into handling of
/// edge cases.
pub fn apply_nested_parser_state<'a, 'b>(accumulator: &'a mut ConundrumInput<'a>, child: &'b ConundrumInput<'b>) {
    let mut state = accumulator.state.borrow_mut();
    let child_state = child.state.borrow();

    state.bib.merge_child_state(&child_state.bib);
}

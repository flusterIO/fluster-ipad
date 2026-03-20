use crate::lang::runtime::traits::conundrum_input::ParseState;

pub fn apply_nested_state(a: &mut ParseState, b: &ParseState) {
    b.data.citations.iter().for_each(|cit| {
                               a.data.citations.push(cit.clone());
                           });
}

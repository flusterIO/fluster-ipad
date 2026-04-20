use crate::{
    lang::runtime::{state::parse_state::ParseState, traits::conundrum_input::ConundrumInput},
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};
use parking_lot::RwLock;
use std::sync::Arc;

pub fn wrap_test_conundrum_content<'a>(content: &'a str) -> ConundrumInput<'a> {
    let state = Arc::new(RwLock::new(ParseState { data: MdxParsingResult::from_initial_mdx_content(content),
                                                  ..Default::default() }));
    ConundrumInput { input: content,
                     state }
}

use std::cell::RefCell;

use crate::{
    lang::runtime::traits::conundrum_input::{ConundrumInput, ParseState},
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
};

pub fn wrap_test_conundrum_content<'a>(content: &'a str) -> ConundrumInput<'a> {
    let state = RefCell::new(ParseState { data: MdxParsingResult::from_initial_mdx_content(content) });
    ConundrumInput { input: content,
                     state }
}

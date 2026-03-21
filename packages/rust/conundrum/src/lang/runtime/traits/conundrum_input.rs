use std::cell::RefCell;

use winnow::Stateful;

use crate::{
    lang::runtime::state::parse_state::ParseState, output::parsing_result::mdx_parsing_result::MdxParsingResult,
};

pub type ConundrumInput<'a> = Stateful<&'a str, RefCell<ParseState>>;

pub fn get_conundrum_input(val: &str) -> ConundrumInput {
    ConundrumInput { input: val,
                     state: RefCell::new(ParseState { data: MdxParsingResult::from_initial_mdx_content(val) }) }
}

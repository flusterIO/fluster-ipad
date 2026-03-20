use std::cell::RefCell;

use winnow::Stateful;

use crate::output::parsing_result::mdx_parsing_result::MdxParsingResult;

#[derive(Debug)]
pub struct ParseState {
    pub data: MdxParsingResult,
}

impl Default for ParseState {
    fn default() -> Self {
        Self { data: Default::default() }
    }
}

pub type ConundrumInput<'a> = Stateful<&'a str, RefCell<ParseState>>;

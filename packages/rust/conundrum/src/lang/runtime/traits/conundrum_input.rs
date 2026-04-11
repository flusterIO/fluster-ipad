use std::cell::RefCell;

use winnow::Stateful;

use crate::{
    lang::runtime::state::{
        citation_list::CitationList,
        parse_state::{ConundrumModifier, ParseState},
    },
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
    parsers::markdown::heading_sluggger::Slugger,
};

pub type ConundrumInput<'a> = Stateful<&'a str, RefCell<ParseState>>;

pub fn get_conundrum_input(val: &str, modifiers: Vec<ConundrumModifier>) -> ConundrumInput {
    ConundrumInput { input: val,
                     state: RefCell::new(ParseState { data: MdxParsingResult::from_initial_mdx_content(val),
                                                      bib: CitationList::default(),
                                                      modifiers,
                                                      eq_count: 0,
                                                      slugger: Slugger::default() }) }
}

pub fn duplicate_conundrum_input(new_value: &str, state: RefCell<ParseState>) -> ConundrumInput {
    ConundrumInput { input: new_value,
                     state }
}

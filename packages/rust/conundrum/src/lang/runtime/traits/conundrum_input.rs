use std::{cell::RefCell, sync::Arc};

use winnow::Stateful;

use crate::{
    lang::runtime::state::{
        citation_list::CitationList,
        dom_data::DomData,
        parse_state::{ConundrumCompileTarget, ConundrumModifier, ParseState},
        ui_params::UIParams,
    },
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
    parsers::markdown::heading_sluggger::Slugger,
};

pub type ConundrumInput<'a> = Stateful<&'a str, Arc<RefCell<ParseState>>>;

pub fn get_conundrum_input(val: &str, state: ParseState) -> ConundrumInput {
    ConundrumInput { input: val,
                     state: Arc::new(RefCell::new(ParseState { data:
                                                                   MdxParsingResult::from_initial_mdx_content(val),
                                                               bib: CitationList::default(),
                                                               modifiers: state.modifiers.clone(),
                                                               eq_count: 0,
                                                               last_heading_depth: 0,
                                                               last_heading_tab_depth: 0,
                                                               valid_footnote_indices: Vec::new(),
                                                               ui_params: state.ui_params.clone(),
                                                               dom: DomData::default(),
                                                               // FIXME: THIS IS BROKEN... This will not
                                                               // work with targets other than html
                                                               // and will break output. This needs to be passed
                                                               // in from the parent state.
                                                               compile_target: state.compile_target.clone(),
                                                               slugger: Slugger::default(),
                                                               trusted: state.trusted.clone() })) }
}

pub fn duplicate_conundrum_input(new_value: &str, state: Arc<RefCell<ParseState>>) -> ConundrumInput {
    ConundrumInput { input: new_value,
                     state }
}
